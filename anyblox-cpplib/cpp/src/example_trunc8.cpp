#include "sysutil.hpp"
#include <fcntl.h>
#include <anyblox.hpp>
#include <memory>
#include <sys/mman.h>

void process_batch(std::shared_ptr<arrow::StructArray> batch,
                   uint32_t *output) {
  if (batch->num_fields() != 1) {
    std::cout << "ARROWERR: not exactly 1 column";
  }
  auto column = batch->field(0);
  std::cout << "ARROW ARRAY: " << column->ToString() << "\n";
  if (column->null_count() != 0) {
    std::cout << "ARROWERR: null count not 0";
  }
  auto int32_array = std::dynamic_pointer_cast<arrow::UInt32Array>(column);
  auto raw_values = int32_array->raw_values();
  auto length_in_bytes = int32_array->length() * 4;

  std::cout << "Copying " << int32_array->length() << " elements\n";

  std::memcpy(output, raw_values, length_in_bytes);
}

int main() {
  auto config_builder = anyblox::config::ConfigBuilder::create();
  config_builder.set_wasm_cache_limit(2LL * 1024 * 1024 * 1024);
  auto config = config_builder.build();

  auto runtime = anyblox::Runtime::create(std::move(config));

  const size_t SIZE = 1024;
  uint8_t *input = new uint8_t[SIZE];
  uint32_t *output = new uint32_t[SIZE];

  for (uint32_t i = 0; i < SIZE; i += 1) {
    input[i] = (uint8_t)((i * i + 7493 * i + 17) % 256);
  }

  auto dataset = sysutil::MemFd({reinterpret_cast<std::byte *>(input), SIZE});
  auto anyblox_bundle = anyblox::AnyBloxBundle::open_extension_and_data(
      "/home/mat/src/portable-decompress/anyblox-cpplib/cpp/res/"
      "trunc8-extension.anyblox",
      dataset.fd(), dataset.len());
  auto metadata = anyblox_bundle.metadata();

  auto params = anyblox::JobParameterBuilder{}.finish(anyblox_bundle);
  auto decode_job = runtime.decode_job_init(std::move(params)).ValueOrDie();
  auto batch = runtime.decode_batch(decode_job, 0, SIZE);

  auto root_type = arrow::struct_(metadata.schema->fields());
  auto array = std::static_pointer_cast<arrow::StructArray>(
      arrow::ImportArray(&batch, root_type).ValueOrDie());
  process_batch(array, output);

  bool ok = true;
  for (uint32_t i = 0; i < SIZE; i += 1) {
    uint32_t expected = (uint32_t)(input[i]);
    uint32_t actual = output[i];

    if (expected != actual) {
      std::cout << "mismatch at " << i << ": original " << expected
                << ", returned " << actual << "\n";
      ok = false;
    }
  }

  if (ok) {
    std::cout << "Everything fine" << std::endl;
  } else {
    std::cout << "Wrong!" << std::endl;
  }

  delete input;
  delete output;
}