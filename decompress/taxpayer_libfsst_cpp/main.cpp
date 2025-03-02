#include "fsst.h"
#include <cstdint>
#include <vector>

class WasmPtr {
public:
  static constexpr WasmPtr null() { return WasmPtr((uint32_t)0); }

  template <typename T>
  explicit WasmPtr(T *ptr) : ptr(reinterpret_cast<uint32_t>(ptr)) {}

private:
  constexpr WasmPtr(uint32_t ptr) : ptr(ptr) {}

  uint32_t ptr;
};

class ColumnProjection {
public:
  const uint32_t MAX_COL_IDX = 63;

  ColumnProjection(uint32_t mask_1, uint32_t mask_2)
      : mask_1(mask_1), mask_2(mask_2) {}

  bool contains(uint32_t col_idx) {
    if (col_idx > MAX_COL_IDX) {
      return false;
    } else if (col_idx < 32) {
      return (mask_1 & (1 << col_idx)) != 0;
    } else {
      return (mask_2 & (1 << (col_idx - 32))) != 0;
    }
  }

private:
  uint32_t mask_1;
  uint32_t mask_2;
};

struct ArrowArray {
  uint32_t length;
  uint32_t null_count;
  uint32_t offset;
  uint32_t n_buffers;
  uint32_t n_children;
  // void** buffers
  WasmPtr buffers;
  // ArrowArray** children
  WasmPtr children;
  // ArrowArray* dictionary
  WasmPtr dictionary;

  static constexpr ArrowArray empty() {
    return ArrowArray{.length = 0,
                      .null_count = 0,
                      .offset = 0,
                      .n_buffers = 0,
                      .n_children = 0,
                      .buffers = WasmPtr::null(),
                      .children = WasmPtr::null(),
                      .dictionary = WasmPtr::null()};
  }
};

ArrowArray batch = ArrowArray::empty();

struct FsstColumn {
  fsst_decoder_t decoder;
  const uint32_t *offsets;
  const unsigned char *data;
};

struct DatasetInfo {
  FsstColumn first_name_column;
  FsstColumn last_name_column;
  FsstColumn state_column;
};

class DecodedColumn {
public:
  DecodedColumn() = default;

  const uint8_t *validity_ptr() const;
  const uint32_t *offsets_ptr() const;
  const unsigned char *data_ptr() const;
  uint32_t get_null_count() const;

  void prepare(uint32_t tuple_count, uint32_t total_compressed_len);
  unsigned char *ptr_for_next_value(uint32_t max_len);
  void push_null();
  void commit_value(size_t len);

  void finish();

private:
  std::vector<unsigned char> data;
  std::vector<uint8_t> validity;
  std::vector<uint32_t> offsets;
  size_t data_offset;
  uint32_t null_count;
  uint8_t validity_byte;
  size_t validity_byte_idx;

  void try_push_validity();
  void force_push_validity();
};

struct State {
  DatasetInfo dataset_info;
  DecodedColumn first_name_col;
  DecodedColumn last_name_col;
  DecodedColumn state_col;

  explicit State(DatasetInfo dataset_info);
};

DatasetInfo read_metadata(const void *data);

void decode_column(DecodedColumn &column, const FsstColumn &compressed,
                   uint32_t start_tuple, uint32_t tuple_count);

void write(uint32_t row_count, const DecodedColumn *first_name,
           const DecodedColumn *last_name, const DecodedColumn *state,
           ArrowArray &batch);

extern "C" void *anyblox_decode(const void *data, uint32_t data_length,
                                 uint32_t start_tuple, uint32_t tuple_count,
                                 void *state, uint32_t projection_mask_1,
                                 uint32_t projection_mask_2) {
  auto state_check_ptr = reinterpret_cast<uint8_t *>(state);
  auto state_ptr = reinterpret_cast<State *>(state_check_ptr + alignof(State));
  if (*state_check_ptr == 0) {
    DatasetInfo dataset_info = read_metadata(data);
    new (state_ptr) State{dataset_info};
    *state_check_ptr = 1;
  }

  auto &state_ref = *state_ptr;
  ColumnProjection projection{projection_mask_1, projection_mask_2};

  DecodedColumn *first_name_d = nullptr;
  DecodedColumn *last_name_d = nullptr;
  DecodedColumn *state_d = nullptr;

  if (projection.contains(0)) {
    decode_column(state_ref.first_name_col,
                  state_ref.dataset_info.first_name_column, start_tuple,
                  tuple_count);
  }
  if (projection.contains(1)) {
    decode_column(state_ref.last_name_col,
                  state_ref.dataset_info.last_name_column, start_tuple,
                  tuple_count);
  }
  if (projection.contains(2)) {
    decode_column(state_ref.state_col, state_ref.dataset_info.state_column,
                  start_tuple, tuple_count);
  }

  write(tuple_count, first_name_d, last_name_d, state_d, batch);

  return &batch;
}

void decode_column(DecodedColumn &column, const FsstColumn &compressed,
                   uint32_t start_tuple, uint32_t tuple_count) {
  uint32_t offset_of_first = compressed.offsets[start_tuple];
  uint32_t offset_of_after_last = compressed.offsets[start_tuple + tuple_count];
  column.prepare(tuple_count, offset_of_after_last - offset_of_first);

  for (size_t tuple_idx = start_tuple, limit = start_tuple + tuple_count;
       tuple_idx < limit; tuple_idx += 1) {
    uint32_t start_offset = compressed.offsets[tuple_idx];
    uint32_t end_offset = compressed.offsets[tuple_idx + 1];
    uint32_t compressed_len = end_offset - start_offset;

    if (compressed_len == 0) {
      column.push_null();
    } else {
      unsigned char *ptr = column.ptr_for_next_value(compressed_len * 8);
      size_t written = fsst_decompress(&compressed.decoder, compressed_len,
                                       compressed.data + start_offset,
                                       compressed_len * 8, ptr);
      column.commit_value(written);
    }
  }

  column.finish();
}

FsstColumn read_column_metadata(const void *data, uint32_t start,
                                uint32_t row_count);

DatasetInfo read_metadata(const void *data) {
  // The header is four 32bit integers: row_count, col1 end, col2 end, col3 end.
  auto data_header = reinterpret_cast<const uint32_t *>(data);
  auto row_count = data_header[0];
  auto first_name_start_offset = data_header[1];
  auto last_name_start_offset = data_header[1];
  auto state_start_offset = data_header[1];

  return DatasetInfo{
      .first_name_column =
          read_column_metadata(data, first_name_start_offset, row_count),
      .last_name_column =
          read_column_metadata(data, last_name_start_offset, row_count),
      .state_column = read_column_metadata(data, state_start_offset, row_count),
  };
}

uint32_t align_64(uint32_t offset) {
  return offset % 64 == 0 ? offset : (offset + (64 - offset % 64));
}

FsstColumn read_column_metadata(const void *data, uint32_t start,
                                uint32_t row_count) {
  // First is the symbol table.
  auto symbol_ptr = reinterpret_cast<const unsigned char *>(data) + start;
  fsst_decoder_t decoder;
  uint32_t symbol_len = fsst_import(&decoder, symbol_ptr);
  uint32_t offsets_offset = align_64(start + symbol_len);

  // Then we have (row_count + 1) offsets.
  auto data_byte_ptr = reinterpret_cast<const unsigned char *>(data);
  auto offsets = reinterpret_cast<const uint32_t *>(data_byte_ptr + offsets_offset);
  uint32_t data_offset = align_64(offsets_offset + 4 * row_count + 4);
  // And finally the actual data.
  auto string_data =
      reinterpret_cast<const unsigned char *>(data_byte_ptr + data_offset);

  return FsstColumn{
      .decoder = decoder, .offsets = offsets, .data = string_data};
}

void write(uint32_t row_count, const DecodedColumn *first_name,
           const DecodedColumn *last_name, const DecodedColumn *state,
           ArrowArray &batch) {
  static WasmPtr BUFFER_ARRAYS[3][3] = {
      {WasmPtr::null(), WasmPtr::null(), WasmPtr::null()},
      {WasmPtr::null(), WasmPtr::null(), WasmPtr::null()},
      {WasmPtr::null(), WasmPtr::null(), WasmPtr::null()},
  };
  static ArrowArray INNER_BATCHES[3] = {
      ArrowArray::empty(),
      ArrowArray::empty(),
      ArrowArray::empty(),
  };
  static WasmPtr CHILDREN_ARRAY[3] = {
      WasmPtr{INNER_BATCHES},
      WasmPtr{INNER_BATCHES + 1},
      WasmPtr{INNER_BATCHES + 2},
  };
  static WasmPtr TOP_LEVEL_BUFFER_ARRAY[1] = {WasmPtr::null()};

  batch.length = row_count;
  batch.null_count = 0;
  batch.n_buffers = 1;
  batch.buffers = WasmPtr{TOP_LEVEL_BUFFER_ARRAY};
  batch.children = WasmPtr{CHILDREN_ARRAY};
  batch.dictionary = WasmPtr::null();

  auto write_column = [&row_count](size_t idx, const DecodedColumn &column) {
    BUFFER_ARRAYS[idx][0] = WasmPtr{column.validity_ptr()};
    BUFFER_ARRAYS[idx][1] = WasmPtr{column.offsets_ptr()};
    BUFFER_ARRAYS[idx][2] = WasmPtr{column.data_ptr()};

    INNER_BATCHES[idx].length = row_count;
    INNER_BATCHES[idx].null_count = column.get_null_count();
    INNER_BATCHES[idx].n_buffers = 3;
    INNER_BATCHES[idx].n_children = 0;
    INNER_BATCHES[idx].buffers = WasmPtr{BUFFER_ARRAYS + idx};
    INNER_BATCHES[idx].children = WasmPtr::null();
    INNER_BATCHES[idx].dictionary = WasmPtr::null();
  };

  size_t idx = 0;
  if (first_name != nullptr) {
    write_column(idx, *first_name);
    idx += 1;
  }
  if (last_name != nullptr) {
    write_column(idx, *last_name);
    idx += 1;
  }
  if (state != nullptr) {
    write_column(idx, *state);
    idx += 1;
  }
  batch.n_children = idx;
}

State::State(DatasetInfo dataset_info)
    : dataset_info(std::move(dataset_info)), first_name_col{}, last_name_col{},
      state_col{} {}

const uint8_t *DecodedColumn::validity_ptr() const { return validity.data(); }

const uint32_t *DecodedColumn::offsets_ptr() const { return offsets.data(); }

const unsigned char *DecodedColumn::data_ptr() const { return data.data(); }

uint32_t DecodedColumn::get_null_count() const { return null_count; }

void DecodedColumn::prepare(uint32_t tuple_count,
                            uint32_t total_compressed_len) {
  data.clear();
  data.reserve(2 * total_compressed_len);
  data_offset = 0;
  validity.clear();
  validity.reserve((tuple_count + 7) / 8);
  offsets.clear();
  offsets.reserve(tuple_count);
  null_count = 0;
  validity_byte = 0;
  validity_byte_idx = 0;
  offsets.push_back(0);
}

unsigned char *DecodedColumn::ptr_for_next_value(uint32_t max_len) {
  size_t start = data_offset;
  size_t end = data_offset + max_len;
  if (end >= data.size()) {
    data.resize(end + 8);
  }

  return data.data() + start;
}

void DecodedColumn::push_null() {
  validity_byte_idx += 1;
  null_count += 1;

  try_push_validity();
  offsets.push_back(data_offset);
}

void DecodedColumn::commit_value(size_t len) {
  validity_byte |= (1 << validity_byte_idx);
  validity_byte_idx += 1;

  try_push_validity();

  data_offset += len;
  offsets.push_back(data_offset);
}

void DecodedColumn::finish() {
  if (validity_byte_idx != 0) {
    force_push_validity();
  }
}

void DecodedColumn::try_push_validity() {
  if (validity_byte_idx == 8) {
    force_push_validity();
  }
}

void DecodedColumn::force_push_validity() {
  validity.push_back(validity_byte);
  validity_byte = 0;
  validity_byte_idx = 0;
}
