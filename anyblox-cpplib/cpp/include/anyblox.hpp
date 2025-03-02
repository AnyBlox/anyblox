#pragma once

#include "gen/anyblox_ffi.hpp"
#include <iomanip>
#include <iostream>

#ifdef NDEBUG
#include <arrow/api.h>
#include <arrow/c/bridge.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#else
#define dynamic_cast static_cast
#include <arrow/api.h>
#include <arrow/c/bridge.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#undef dynamic_cast
#endif

namespace anyblox {
typedef std::tuple<std::byte *, size_t> MemSlice;

// Forward declarations.
class ThreadLocalDecodeJob;
class AnyBloxBundle;
namespace config {
class Config;
class ConfigBuilder;
} // namespace config
// namespace config

class ColumnProjection {
public:
  void add(size_t index);
  uint64_t raw_mask() const;

  ColumnProjection() = default;

  static ColumnProjection empty() { return {}; }
  static ColumnProjection all(size_t column_count);
  template <typename I>
  static ColumnProjection from_indices(const I &begin, const I &end) {
    auto result = empty();
    for (auto iter = begin; iter < end; iter++) {
      result.add(*iter);
    }
    return result;
  }

private:
  uint64_t mask;

  explicit ColumnProjection(uint64_t mask);
};

class Runtime {
public:
  arrow::Result<std::unique_ptr<ThreadLocalDecodeJob>>
  decode_job_init(class JobParameters params);
  ArrowArray decode_batch(std::unique_ptr<ThreadLocalDecodeJob> &job,
                          size_t first_tuple, size_t max_tuples);

  static Runtime create(config::Config config);
  ~Runtime();

  Runtime(const Runtime &) = delete;
  Runtime &operator=(const Runtime &) = delete;
  Runtime(Runtime &&other) { *this = std::move(other); }
  Runtime &operator=(Runtime &&other);

private:
  ffi::AnyBloxRuntime *_inner;

  Runtime(ffi::AnyBloxRuntime *inner);
};

class JobParameters {
private:
  JobParameters(const AnyBloxBundle &);

  std::optional<ColumnProjection> projection;
  bool validate_utf8;
  const AnyBloxBundle &bundle;

  friend class Runtime;
  friend class JobParameterBuilder;
};

class JobParameterBuilder {
public:
  JobParameterBuilder();
  JobParameterBuilder(const JobParameterBuilder &);
  JobParameterBuilder &operator=(const JobParameterBuilder &other) {
    *this = JobParameterBuilder(other);
    return *this;
  }
  JobParameterBuilder(JobParameterBuilder &&);
  JobParameterBuilder &operator=(JobParameterBuilder &&other) {
    *this = JobParameterBuilder(std::move(other));
    return *this;
  };

  JobParameterBuilder &do_not_validate_utf8();
  JobParameterBuilder &with_column_projection(ColumnProjection projection);

  JobParameters finish(const AnyBloxBundle &bundle);

private:
  std::optional<ColumnProjection> _projection;
  bool _validate_utf8;
};

class ThreadLocalDecodeJob {
public:
  std::shared_ptr<arrow::Schema> schema() const;

  ~ThreadLocalDecodeJob();

  ThreadLocalDecodeJob(const ThreadLocalDecodeJob &) = delete;
  ThreadLocalDecodeJob &operator=(const ThreadLocalDecodeJob &) = delete;
  ThreadLocalDecodeJob(ThreadLocalDecodeJob &&);
  ThreadLocalDecodeJob &operator=(ThreadLocalDecodeJob &&other) {
    *this = ThreadLocalDecodeJob(std::move(other));
    return *this;
  };

private:
  std::shared_ptr<arrow::Schema> _schema;
  ffi::AnyBloxJobContext *_inner;

  ThreadLocalDecodeJob(ffi::AnyBloxJobInit job_init,
                       std::shared_ptr<arrow::Schema> schema);

  friend class Runtime;
};

struct AnyBloxMetadata {
  struct Decoder {
    std::string uri;
    std::optional<std::string> description;
    std::optional<std::string> license;
    std::optional<std::string> checksum_blake3;
    std::optional<uint64_t> min_batch_size;
  };

  struct Data {
    std::string name;
    uint64_t count;
    std::optional<uint64_t> size_in_bytes;
    std::optional<std::string> description;
  };

  std::shared_ptr<arrow::Schema> schema;
  Decoder decoder;
  Data data;
};

typedef int Fd;

class AnyBloxBundle {
public:
  bool is_extension() const;
  bool is_self_contained() const;

  MemSlice decoder() const;
  const AnyBloxMetadata &metadata() const;

  static AnyBloxBundle open_self_contained(std::string path);
  static AnyBloxBundle open_self_contained(Fd data, size_t len);
  static AnyBloxBundle open_extension_and_data(std::string anyblox_path,
                                                std::string data_path);
  static AnyBloxBundle open_extension_and_data(Fd anyblox,
                                                size_t anyblox_len,
                                                std::string data_path);
  static AnyBloxBundle open_extension_and_data(std::string anyblox_path,
                                                Fd data, size_t data_len);
  static AnyBloxBundle open_extension_and_data(Fd anyblox,
                                                size_t anyblox_len, Fd data,
                                                size_t data_len);
  AnyBloxBundle() = delete;
  ~AnyBloxBundle();

  AnyBloxBundle(const AnyBloxBundle &) = delete;
  AnyBloxBundle &operator=(const AnyBloxBundle &) = delete;
  AnyBloxBundle(AnyBloxBundle &&);
  AnyBloxBundle &operator=(AnyBloxBundle &&);

private:
  enum class Type { Extension, SelfContained };

  Type _type;
  mutable std::optional<AnyBloxMetadata> _metadata;
  ffi::AnyBloxBundle *_inner;

  AnyBloxBundle(ffi::AnyBloxBundle *inner, Type type);

  friend class Runtime;
};

namespace config {
class Config {
public:
  ~Config();

  Config(const Config &) = delete;
  Config &operator=(const Config &) = delete;
  Config(Config &&other) { *this = std::move(other); }
  Config &operator=(Config &&other);

private:
  ffi::AnyBloxConfig *_inner;

  Config(ffi::AnyBloxConfig *inner);
  friend class ConfigBuilder;
  friend anyblox::Runtime anyblox::Runtime::create(Config config);
};

enum class LogLevel {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4,
};

class ConfigBuilder {
public:
  ConfigBuilder *compile_with_debug(bool value);
  ConfigBuilder *set_log_level(LogLevel level);
  ConfigBuilder *set_log_directory(std::string &directory);
  ConfigBuilder *set_oltp_collector_endpoint(std::string &endpoint);
  ConfigBuilder *set_wasm_cache_limit(size_t limit);
  ConfigBuilder *set_thread_virtual_memory_limit(size_t limit);

  Config build();

  static ConfigBuilder create();
  ~ConfigBuilder();

  ConfigBuilder(const ConfigBuilder &) = delete;
  ConfigBuilder &operator=(const ConfigBuilder &) = delete;
  ConfigBuilder(ConfigBuilder &&other) { *this = std::move(other); }
  ConfigBuilder &operator=(ConfigBuilder &&other);

private:
  ffi::AnyBloxConfigBuilder *_inner;

  ConfigBuilder(ffi::AnyBloxConfigBuilder *inner);
};
} // namespace config
} // namespace anyblox
