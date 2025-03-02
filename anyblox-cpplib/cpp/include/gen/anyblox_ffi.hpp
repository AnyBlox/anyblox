#pragma once

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace anyblox {
namespace ffi {

constexpr static const size_t ColumnProjection_MAX_COL_IDX = 63;

constexpr static const uint64_t FsstDecoder_FSST_VERSION = 20190218;

constexpr static const uint64_t FsstDecoder_FSST_CORRUPT = 32774747032022883;

constexpr static const uint8_t FsstDecoder_FSST_ESC = 255;

enum class LogLevel {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4,
};

struct AnyBloxBundle;

struct AnyBloxConfig;

struct AnyBloxConfigBuilder;

struct AnyBloxJobContext;

struct AnyBloxRuntime;

struct DataMetadata;

struct DecoderMetadata;

struct MemoryRank;

struct Schema;

struct MemSlice {
  uint8_t *ptr;
  size_t len;
};

struct AnyBloxJobInit {
  AnyBloxJobContext *job_context;
};

struct AnyBloxRecordBatch {
  int64_t length;
  int64_t null_count;
  int64_t offset;
  int64_t n_buffers;
  int64_t n_children;
  const void **buffers;
  AnyBloxRecordBatch **children;
  AnyBloxRecordBatch *dictionary;
  void (*release)(AnyBloxRecordBatch *arg1);
  void *private_data;
};

struct AnyBloxMetadata {
  const Schema *schema;
  const DecoderMetadata *decoder_metadata;
  const DataMetadata *data_metadata;
};

struct SchemaDataType {
  enum class Tag {
    Null,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Date32,
    Date64,
    Binary,
    LargeBinary,
    BinaryView,
    Utf8,
    LargeUtf8,
    Utf8View,
    FixedSizeBinary,
  };

  struct FixedSizeBinary_Body {
    int32_t _0;
  };

  Tag tag;
  union {
    FixedSizeBinary_Body fixed_size_binary;
  };
};

template<typename T>
struct COption {
  enum class Tag {
    Some,
    None,
  };

  struct Some_Body {
    T _0;
  };

  Tag tag;
  union {
    Some_Body some;
  };
};



extern "C" {

AnyBloxConfigBuilder *config_builder_create();

void config_builder_drop(AnyBloxConfigBuilder *builder);

void config_builder_set_wasm_cache_limit(AnyBloxConfigBuilder *builder, size_t limit);

void config_builder_set_thread_virtual_memory_limit(AnyBloxConfigBuilder *builder, size_t limit);

void config_builder_set_log_level(AnyBloxConfigBuilder *builder, LogLevel log_level);

void config_builder_set_log_directory(AnyBloxConfigBuilder *builder, MemSlice directory);

void config_builder_compile_with_debug(AnyBloxConfigBuilder *builder, bool value);

void config_builder_set_oltp_collector_endpoint(AnyBloxConfigBuilder *builder, MemSlice endpoint);

AnyBloxConfig *config_builder_finish(AnyBloxConfigBuilder *builder);

void config_drop(AnyBloxConfig *config);

AnyBloxRuntime *runtime_create(AnyBloxConfig *config);

AnyBloxJobInit runtime_decode_init(AnyBloxRuntime *runtime,
                                   AnyBloxBundle *input,
                                   bool validate_utf8);

AnyBloxJobInit runtime_decode_init_with_projection(AnyBloxRuntime *runtime,
                                                   AnyBloxBundle *input,
                                                   uint64_t projection,
                                                   bool validate_utf8);

void runtime_drop(AnyBloxRuntime *runtime);

AnyBloxRecordBatch job_run_and_block(AnyBloxRuntime *runtime,
                                     AnyBloxJobContext *job_context,
                                     size_t first_tuple,
                                     size_t tuple_count);

void job_drop(AnyBloxJobContext *context);

AnyBloxBundle *bundle_open_extension(int anyblox_fd,
                                     size_t anyblox_len,
                                     int dataset_fd,
                                     size_t dataset_len);

AnyBloxBundle *bundle_open_self_contained(int fd, size_t len);

MemSlice bundle_decoder(const AnyBloxBundle *bundle);

AnyBloxMetadata bundle_metadata(const AnyBloxBundle *bundle);

void bundle_drop(AnyBloxBundle *bundle);

size_t schema_fields_count(const AnyBloxMetadata *metadata);

MemSlice schema_field_name(const AnyBloxMetadata *metadata, size_t field_idx);

SchemaDataType schema_field_datatype(const AnyBloxMetadata *metadata, size_t field_idx);

bool schema_field_nullable(const AnyBloxMetadata *metadata, size_t field_idx);

MemSlice decoder_metadata_uri(const AnyBloxMetadata *metadata);

COption<MemSlice> decoder_metadata_description(const AnyBloxMetadata *metadata);

COption<MemSlice> decoder_metadata_license(const AnyBloxMetadata *metadata);

COption<MemSlice> decoder_metadata_checksum_blake3(const AnyBloxMetadata *metadata);

COption<uint64_t> decoder_metadata_min_batch_size(const AnyBloxMetadata *metadata);

MemSlice data_metadata_name(const AnyBloxMetadata *metadata);

uint64_t data_metadata_count(const AnyBloxMetadata *metadata);

COption<MemSlice> data_metadata_description(const AnyBloxMetadata *metadata);

COption<uint64_t> data_metadata_size_in_bytes(const AnyBloxMetadata *metadata);

} // extern "C"

} // namespace ffi
} // namespace anyblox
