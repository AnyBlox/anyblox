#include "anyblox.hpp"
#include "ffi_utils.hpp"
#include "sysutil.hpp"
#include <fstream>

namespace anyblox {
namespace config {
ConfigBuilder::ConfigBuilder(ffi::AnyBloxConfigBuilder *inner)
    : _inner(inner) {}
ConfigBuilder::~ConfigBuilder() {
  if (_inner != nullptr) {
    ffi::config_builder_drop(_inner);
  }
}
ConfigBuilder &ConfigBuilder::operator=(ConfigBuilder &&other) {
  this->_inner = other._inner;
  other._inner = nullptr;
  return *this;
}
ConfigBuilder *ConfigBuilder::compile_with_debug(bool value) {
  ffi::config_builder_compile_with_debug(_inner, value);
  return this;
}
ConfigBuilder *ConfigBuilder::set_wasm_cache_limit(size_t limit) {
  ffi::config_builder_set_wasm_cache_limit(_inner, limit);
  return this;
}
ConfigBuilder *ConfigBuilder::set_thread_virtual_memory_limit(size_t limit) {
  ffi::config_builder_set_thread_virtual_memory_limit(_inner, limit);
  return this;
}
ConfigBuilder *ConfigBuilder::set_log_level(LogLevel logLevel) {
  ffi::config_builder_set_log_level(
      _inner, ffi::convert(logLevel).into<ffi::LogLevel>());
  return this;
}
ConfigBuilder *ConfigBuilder::set_log_directory(std::string &directory) {
  ffi::config_builder_set_log_directory(
      _inner, ffi::Convert<std::string &>(directory).into<ffi::MemSlice>());
  return this;
}
ConfigBuilder *
ConfigBuilder::set_oltp_collector_endpoint(std::string &directory) {
  ffi::config_builder_set_log_directory(
      _inner, ffi::Convert<std::string &>(directory).into<ffi::MemSlice>());
  return this;
}

Config ConfigBuilder::build() {
  auto inner_config = ffi::config_builder_finish(_inner);
  _inner = nullptr;
  return Config{inner_config};
}

ConfigBuilder ConfigBuilder::create() {
  auto inner = ffi::config_builder_create();
  return ConfigBuilder{inner};
}

Config::Config(ffi::AnyBloxConfig *inner) : _inner(inner) {}
Config::~Config() {
  if (_inner != nullptr) {
    ffi::config_drop(_inner);
  }
}
Config &Config::operator=(Config &&other) {
  this->_inner = other._inner;
  other._inner = nullptr;
  return *this;
}
} // namespace config

ColumnProjection::ColumnProjection(uint64_t mask) : mask(mask) {}

ColumnProjection ColumnProjection::all(size_t count) {
  if (count > 64) {
    throw std::invalid_argument(
        "Maximum number of columns in ColumnProjection::all is 64, requested " +
        std::to_string(count));
  }
  if (count == 64) {
    return ColumnProjection{0xFFFF'FFFF'FFFF'FFFFull};
  }
  return ColumnProjection{(1ull << count) - 1};
}

void ColumnProjection::add(size_t idx) {
  if (idx >= 64) {
    throw std::invalid_argument(
        "Maximum number of columns in ColumnProjection::all is 64, requested "
        "index " +
        std::to_string(idx));
  }
  mask |= (1ull << idx);
}

uint64_t ColumnProjection::raw_mask() const { return mask; }

Runtime::Runtime(ffi::AnyBloxRuntime *inner) : _inner(inner) {}
Runtime::~Runtime() {
  if (_inner != nullptr) {
    ffi::runtime_drop(_inner);
  }
}
Runtime &Runtime::operator=(Runtime &&other) {
  this->_inner = other._inner;
  other._inner = nullptr;
  return *this;
}

arrow::Result<std::unique_ptr<ThreadLocalDecodeJob>>
Runtime::decode_job_init(JobParameters params) {
  auto schema = params.bundle.metadata().schema;
  auto job =
      params.projection
          ? ffi::runtime_decode_init_with_projection(
                _inner, params.bundle._inner,
                params.projection.value().raw_mask(), params.validate_utf8)
          : ffi::runtime_decode_init(_inner, params.bundle._inner,
                                     params.validate_utf8);

  return std::unique_ptr<ThreadLocalDecodeJob>(
      new ThreadLocalDecodeJob(job, std::move(schema)));
}

ArrowArray Runtime::decode_batch(std::unique_ptr<ThreadLocalDecodeJob> &job,
                                 size_t first_tuple, size_t max_tuples) {
  anyblox::ffi::AnyBloxRecordBatch batch = ffi::job_run_and_block(
      this->_inner, job->_inner, first_tuple, max_tuples);

  return *(reinterpret_cast<ArrowArray *>(&batch));
}

Runtime Runtime::create(config::Config config) {
  auto inner = ffi::runtime_create(config._inner);
  config._inner = nullptr;
  return Runtime{inner};
}

ThreadLocalDecodeJob::ThreadLocalDecodeJob(
    ffi::AnyBloxJobInit job_init, std::shared_ptr<arrow::Schema> schema)
    : _inner(job_init.job_context), _schema(std::move(schema)) {}
ThreadLocalDecodeJob::~ThreadLocalDecodeJob() {
  if (_inner != nullptr) {
    ffi::job_drop(_inner);
  }
}
ThreadLocalDecodeJob::ThreadLocalDecodeJob(ThreadLocalDecodeJob &&other) {
  this->_inner = std::move(other._inner);
  this->_schema = std::move(other._schema);
  other._inner = nullptr;
}

std::shared_ptr<arrow::Schema> ThreadLocalDecodeJob::schema() const {
  return this->_schema;
}

bool AnyBloxBundle::is_extension() const {
  return _type == AnyBloxBundle::Type::Extension;
}

bool AnyBloxBundle::is_self_contained() const {
  return _type == AnyBloxBundle::Type::SelfContained;
}

MemSlice AnyBloxBundle::decoder() const {
  ffi::MemSlice slice = ffi::bundle_decoder(this->_inner);
  return ffi::convert(slice).into<MemSlice>();
}

const AnyBloxMetadata &AnyBloxBundle::metadata() const {
  if (_metadata) {
    return _metadata.value();
  }

  ffi::AnyBloxMetadata metadata = ffi::bundle_metadata(this->_inner);

  size_t fields_count = ffi::schema_fields_count(&metadata);
  arrow::SchemaBuilder schema_builder{};
  for (size_t i = 0; i < fields_count; i += 1) {
    auto field = make_shared<arrow::Field>(
        ffi::convert(ffi::schema_field_name(&metadata, i)).into<std::string>(),
        ffi::convert(ffi::schema_field_datatype(&metadata, i))
            .into<std::shared_ptr<arrow::DataType>>(),
        ffi::schema_field_nullable(&metadata, i));
    auto status = schema_builder.AddField(field);
    assert(status.ok());
  }

  auto schema = schema_builder.Finish().ValueOrDie();
  auto decoder = AnyBloxMetadata::Decoder{
      .uri = ffi::convert(ffi::decoder_metadata_uri(&metadata))
                 .into<std::string>(),
      .description = ffi::convert(ffi::decoder_metadata_description(&metadata))
                         .map_into<std::string>(),
      .license = ffi::convert(ffi::decoder_metadata_license(&metadata))
                     .map_into<std::string>(),
      .checksum_blake3 =
          ffi::convert(ffi::decoder_metadata_checksum_blake3(&metadata))
              .map_into<std::string>(),
      .min_batch_size =
          ffi::convert(ffi::decoder_metadata_min_batch_size(&metadata)).into(),
  };
  auto data = AnyBloxMetadata::Data{
      .name =
          ffi::convert(ffi::data_metadata_name(&metadata)).into<std::string>(),
      .count = ffi::data_metadata_count(&metadata),
      .size_in_bytes =
          ffi::convert(ffi::data_metadata_size_in_bytes(&metadata)).into(),
      .description = ffi::convert(ffi::data_metadata_description(&metadata))
                         .map_into<std::string>()};

  _metadata =
      AnyBloxMetadata{.schema = schema, .decoder = decoder, .data = data};
  return _metadata.value();
}

AnyBloxBundle AnyBloxBundle::open_self_contained(std::string path) {
  Fd fd = sysutil::get_fd(std::move(path));
  size_t len = sysutil::get_file_len(fd);
  return open_self_contained(fd, len);
}

AnyBloxBundle AnyBloxBundle::open_self_contained(Fd fd, size_t len) {
  ffi::AnyBloxBundle *inner = ffi::bundle_open_self_contained(fd, len);
  return AnyBloxBundle{inner, AnyBloxBundle::Type::SelfContained};
}

AnyBloxBundle AnyBloxBundle::open_extension_and_data(std::string anyblox_path,
                                                     std::string data_path) {
  Fd anyblox_fd = sysutil::get_fd(std::move(anyblox_path));
  size_t anyblox_len = sysutil::get_file_len(anyblox_fd);
  return open_extension_and_data(anyblox_fd, anyblox_len, std::move(data_path));
}
AnyBloxBundle AnyBloxBundle::open_extension_and_data(Fd anyblox,
                                                     size_t anyblox_len,
                                                     std::string data_path) {
  Fd data_fd = sysutil::get_fd(std::move(data_path));
  size_t data_len = sysutil::get_file_len(data_fd);
  return open_extension_and_data(anyblox, anyblox_len, data_fd, data_len);
}
AnyBloxBundle AnyBloxBundle::open_extension_and_data(std::string anyblox_path,
                                                     Fd data, size_t data_len) {
  Fd anyblox_fd = sysutil::get_fd(std::move(anyblox_path));
  size_t anyblox_len = sysutil::get_file_len(anyblox_fd);
  return open_extension_and_data(anyblox_fd, anyblox_len, data, data_len);
}
AnyBloxBundle AnyBloxBundle::open_extension_and_data(Fd anyblox,
                                                     size_t anyblox_len,
                                                     Fd data, size_t data_len) {
  ffi::AnyBloxBundle *inner =
      ffi::bundle_open_extension(anyblox, anyblox_len, data, data_len);
  return AnyBloxBundle{inner, AnyBloxBundle::Type::Extension};
}

AnyBloxBundle::AnyBloxBundle(ffi::AnyBloxBundle *inner, Type type)
    : _inner(inner), _type(type) {}

AnyBloxBundle::~AnyBloxBundle() {
  if (_inner != nullptr) {
    ffi::bundle_drop(_inner);
    _inner = nullptr;
  }
}

AnyBloxBundle &AnyBloxBundle::operator=(AnyBloxBundle &&other) {
  this->_inner = std::move(other._inner);
  this->_type = std::move(other._type);
  this->_metadata = std::move(other._metadata);
  other._inner = nullptr;
  return *this;
}
AnyBloxBundle::AnyBloxBundle(AnyBloxBundle &&other) {
  *this = std::move(other);
}

JobParameters::JobParameters(const AnyBloxBundle &bundle) : bundle(bundle) {}
JobParameterBuilder::JobParameterBuilder() = default;
JobParameterBuilder::JobParameterBuilder(const JobParameterBuilder &) = default;
anyblox::JobParameterBuilder::JobParameterBuilder(JobParameterBuilder &&) =
    default;

JobParameterBuilder &anyblox::JobParameterBuilder::do_not_validate_utf8() {
  _validate_utf8 = false;
  return *this;
}

JobParameterBuilder &anyblox::JobParameterBuilder::with_column_projection(
    ColumnProjection projection) {
  _projection = projection;
  return *this;
}

JobParameters
anyblox::JobParameterBuilder::finish(const AnyBloxBundle &bundle) {
  JobParameters params{bundle};
  params.projection = _projection;
  params.validate_utf8 = _validate_utf8;
  return params;
}
} // namespace anyblox
