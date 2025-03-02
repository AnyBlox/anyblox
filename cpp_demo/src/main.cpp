#include "anyblox.h"
#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <iomanip>
#include <iostream>

template <typename T> void hex_print(T &data) {
  std::cout << "data length = " << data.size() << "\n";
  size_t count = 0;
  for (uint8_t byte : data) {
    std::cout << std::setw(2) << std::setfill('0') << std::hex << "0x"
              << (uint32_t)(byte) << " ";
    count += 1;
    if (count == 16) {
      std::cout << "\n";
      count = 0;
    }
  }
  std::cout << std::dec << "\n";
}

class AnyBloxMessageReader : public arrow::ipc::MessageReader {
public:
  AnyBloxMessageReader(anyblox::AnyBloxRuntime &runtime)
      : runtime(runtime) {}

  arrow::Result<std::unique_ptr<arrow::ipc::Message>>
  ReadNextMessage() override {
    // Schema bytes must be kept alive, so we save it internally.
    schema = runtime.get_schema();
    hex_print(schema);
    ARROW_RETURN_NOT_OK(validateContinuation(schema.data(), schema.size()));
    ARROW_ASSIGN_OR_RAISE(uint32_t message_size,
                          readInt32(schema.data() + 4, schema.size() - 4));
    auto schema_buffer =
        std::make_shared<arrow::Buffer>(schema.data() + 8, message_size);
    auto message = arrow::ipc::Message::Open(schema_buffer, nullptr);
    return message;
  }

private:
  anyblox::AnyBloxRuntime &runtime;

  rust::cxxbridge1::Vec<uint8_t> schema;

  static arrow::Status validateContinuation(uint8_t *data, size_t size) {
    ARROW_ASSIGN_OR_RAISE(uint32_t marker, readInt32(data, size));
    if (marker != 0xFFFFFFFF) {
      return arrow::Status::IOError(
          "invalid IPC message not starting with the continuation marker");
    }
    return arrow::Status::OK();
  }

  static arrow::Result<uint32_t> readInt32(uint8_t *data, size_t size) {
    if (size < sizeof(uint32_t)) {
      return arrow::Status::IOError("invalid IPC message - not enough bytes");
    }
    uint32_t value = *reinterpret_cast<uint32_t *>(data);
    return value;
  }
};

int main() {
  auto config_builder = anyblox::config::create_builder();
  config_builder->set_wasm_cache_limit(2LL * 1024 * 1024 * 1024)
      .set_wasm_dir("/home/mat/src/portable-decompress/decompress/target/"
                    "wasm32-unknown-unknown/release");
  auto runtime = anyblox::create_runtime(
      anyblox::config::build_config(std::move(config_builder)));
  std::unique_ptr<arrow::ipc::MessageReader> message_reader =
      std::make_unique<AnyBloxMessageReader>(*runtime);
  auto record_batch_reader =
      arrow::ipc::RecordBatchStreamReader::Open(std::move(message_reader));

  auto schema = record_batch_reader->get()->schema();

  std::cout << schema->ToString(true) << std::endl;

  auto batch = record_batch_reader->get()->Next().ValueOrDie();
  std::cout << batch->ToString() << std::endl;
}