#include "sysutil.hpp"
#include <fcntl.h>
#include <fstream>
#include <anyblox.hpp>
#include <memory>
#include <sys/mman.h>

class CastToString {
public:
  std::unique_ptr<arrow::StringArray> array;

  arrow::Status Visit(const arrow::Array &array) {
    return arrow::Status::NotImplemented("expected a string array not ",
                                         array.type()->ToString());
  }

  arrow::Status Visit(const arrow::StringArray &array) {
    this->array = std::make_unique<arrow::StringArray>(array.data());
    return {};
  }

  static arrow::Result<std::unique_ptr<arrow::StringArray>>
  cast(std::shared_ptr<arrow::Array> array) {
    CastToString instance{};
    auto status = arrow::VisitArrayInline(*array, &instance);
    if (status.ok()) {
      return std::move(instance.array);
    } else {
      return status;
    }
  }
};

int main() {
  auto config_builder = anyblox::config::ConfigBuilder::create();
  config_builder.set_wasm_cache_limit(2LL * 1024 * 1024 * 1024);
  auto config = config_builder.build();

  auto runtime = anyblox::Runtime::create(std::move(config));

  auto anyblox_bundle = anyblox::AnyBloxBundle::open_self_contained(
      "/home/mat/src/portable-decompress/anyblox-cpplib/cpp/res/"
      "taxpayer-sample-self-contained.anyblox");
  auto metadata = anyblox_bundle.metadata();

  auto params = anyblox::JobParameterBuilder{}.finish(anyblox_bundle);
  auto decode_job = runtime.decode_job_init(std::move(params)).ValueOrDie();
  auto batch = runtime.decode_batch(decode_job, 0, metadata.data.count);

  bool ok = true;
  auto csv_stream =
      std::ifstream("/home/mat/src/portable-decompress/anyblox-cpplib/cpp/res/"
                    "taxpayer-sample.csv");

  if (batch.length != metadata.data.count) {
    std::cout << "invalid number of rows returned, expected "
              << metadata.data.count << " got " << batch.length << "\n";
    ok = false;
  }
  if (batch.n_children != 3) {
    std::cout << "invalid number of columns returned, expected 3 got "
              << batch.n_children << "\n";
    ok = false;
  }

  std::vector<std::tuple<std::string, std::string, std::string>> expected;

  {
    std::string csv_line;
    size_t record = 0;
    while (std::getline(csv_stream, csv_line)) {
      std::istringstream csv_stream(csv_line);
      std::vector<std::string> csv_columns;
      {
        std::string csv_elem;
        while (std::getline(csv_stream, csv_elem, '|')) {
          csv_columns.emplace_back(std::move(csv_elem));
          csv_elem.clear();
        }
      }
      assert(csv_columns.size() == 3);
      expected.emplace_back(std::move(csv_columns[0]),
                            std::move(csv_columns[1]),
                            std::move(csv_columns[2]));
    }
  }
  
  auto root_type = arrow::struct_(metadata.schema->fields());
  auto array = std::static_pointer_cast<arrow::StructArray>(arrow::ImportArray(&batch, root_type).ValueOrDie());
  auto first_name_array = CastToString::cast(array->field(0)).ValueOrDie();
  auto last_name_array = CastToString::cast(array->field(1)).ValueOrDie();
  auto state_array = CastToString::cast(array->field(2)).ValueOrDie();

  std::cout << "ARROW ARRAY: " << first_name_array->ToString() << "\n";
  std::cout << "ARROW ARRAY: " << last_name_array->ToString() << "\n";
  std::cout << "ARROW ARRAY: " << state_array->ToString() << "\n";

  for (uint32_t i = 0; i < expected.size(); i += 1) {
    std::string_view first_name = first_name_array->GetView(i);
    std::string_view last_name = last_name_array->GetView(i);
    std::string_view state = state_array->GetView(i);

    if (first_name != std::get<0>(expected[i])) {
      std::cout << "mismatched first_name at " << i << ": original "
                << std::get<0>(expected[i]) << ", returned " << first_name
                << "\n";
      ok = false;
    }
    if (last_name != std::get<1>(expected[i])) {
      std::cout << "mismatched last_name at " << i << ": original "
                << std::get<1>(expected[i]) << ", returned " << last_name
                << "\n";
      ok = false;
    }
    if (state != std::get<2>(expected[i])) {
      std::cout << "mismatched state at " << i << ": original "
                << std::get<2>(expected[i]) << ", returned " << state << "\n";
      ok = false;
    }

    if (first_name.empty() != first_name_array->IsNull(i)) {
      std::cout << "mismatched first_name nullability at " << i << "\n";
      ok = false;
    }
    if (last_name.empty() != last_name_array->IsNull(i)) {
      std::cout << "mismatched last_name nullability at " << i << "\n";
      ok = false;
    }
    if (state.empty() != state_array->IsNull(i)) {
      std::cout << "mismatched state nullability at " << i << "\n";
      ok = false;
    }
  }

  if (ok) {
    std::cout << "Everything fine" << std::endl;
  } else {
    std::cout << "Wrong!" << std::endl;
  }
}