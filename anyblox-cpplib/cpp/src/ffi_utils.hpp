#pragma once
#include "gen/anyblox_ffi.hpp"
#include <arrow/api.h>
#include <arrow/c/bridge.h>
#include <cstdint>
#include <optional>
#include <tuple>

namespace anyblox {
namespace ffi {

template <typename T> struct BaseConvert {
  BaseConvert(T value) : value(value) {}

  T value;
};

template <typename T> struct Convert : BaseConvert<T> {};

template <typename T> struct Convert<COption<T>> : BaseConvert<COption<T>> {
  std::optional<T> into() {
    if (this->value.tag == COption<T>::Tag::None) {
      return {};
    } else {
      return this->value.some._0;
    }
  }

  template <typename TTarget> std::optional<TTarget> map_into() {
    if (this->value.tag == COption<T>::Tag::None) {
      return {};
    } else {
      return convert(this->value.some._0).template into<TTarget>();
    }
  }
};

template <> struct Convert<ffi::MemSlice> : BaseConvert<ffi::MemSlice> {
  template <typename TTarget> TTarget into();
};

template <> struct Convert<std::string &> : BaseConvert<std::string &> {
  template <typename TTarget> TTarget into();
};

template <>
struct Convert<anyblox::config::LogLevel>
    : BaseConvert<anyblox::config::LogLevel> {
  template <typename TTarget> TTarget into();
};

template <>
struct Convert<ffi::SchemaDataType> : BaseConvert<ffi::SchemaDataType> {
  template <typename TTarget> TTarget into();
};

template <>
struct Convert<ffi::AnyBloxRecordBatch>
    : BaseConvert<ffi::AnyBloxRecordBatch> {
  template <typename TTarget>
  TTarget into(std::shared_ptr<arrow::Schema> schema);
};

template <> std::string Convert<ffi::MemSlice>::into<std::string>() {
  return std::string(reinterpret_cast<char *>(this->value.ptr),
                     this->value.len);
}

template <>
std::tuple<std::byte *, size_t>
Convert<ffi::MemSlice>::into<std::tuple<std::byte *, size_t>>() {
  return std::make_tuple(reinterpret_cast<std::byte *>(this->value.ptr),
                         this->value.len);
}

template <> ffi::MemSlice Convert<std::string &>::into<ffi::MemSlice>() {
  return ffi::MemSlice{.ptr = reinterpret_cast<uint8_t *>(value.data()),
                       .len = value.length()};
}

template <>
ffi::LogLevel Convert<anyblox::config::LogLevel>::into<ffi::LogLevel>() {
  using anyblox::config::LogLevel;
  switch (value) {
  case LogLevel::Trace:
    return ffi::LogLevel::Trace;
  case LogLevel::Debug:
    return ffi::LogLevel::Debug;
  case LogLevel::Info:
    return ffi::LogLevel::Info;
  case LogLevel::Warn:
    return ffi::LogLevel::Warn;
  case LogLevel::Error:
    return ffi::LogLevel::Error;
  default:
    throw std::runtime_error("unreachable");
  }
}

template <>
std::shared_ptr<arrow::DataType>
Convert<ffi::SchemaDataType>::into<std::shared_ptr<arrow::DataType>>() {
  switch (this->value.tag) {
  case ffi::SchemaDataType::Tag::Null:
    return arrow::null();
  case ffi::SchemaDataType::Tag::Boolean:
    return arrow::boolean();
  case ffi::SchemaDataType::Tag::Int8:
    return arrow::int8();
  case ffi::SchemaDataType::Tag::Int16:
    return arrow::int16();
  case ffi::SchemaDataType::Tag::Int32:
    return arrow::int32();
  case ffi::SchemaDataType::Tag::Int64:
    return arrow::int64();
  case ffi::SchemaDataType::Tag::UInt8:
    return arrow::uint8();
  case ffi::SchemaDataType::Tag::UInt16:
    return arrow::uint16();
  case ffi::SchemaDataType::Tag::UInt32:
    return arrow::uint32();
  case ffi::SchemaDataType::Tag::UInt64:
    return arrow::uint64();
  case ffi::SchemaDataType::Tag::Float16:
    return arrow::float16();
  case ffi::SchemaDataType::Tag::Float32:
    return arrow::float32();
  case ffi::SchemaDataType::Tag::Float64:
    return arrow::float64();
  case ffi::SchemaDataType::Tag::Date32:
    return arrow::date32();
  case ffi::SchemaDataType::Tag::Date64:
    return arrow::date64();
  case ffi::SchemaDataType::Tag::Binary:
    return arrow::binary();
  case ffi::SchemaDataType::Tag::LargeBinary:
    return arrow::large_binary();
  case ffi::SchemaDataType::Tag::BinaryView:
    return arrow::binary_view();
  case ffi::SchemaDataType::Tag::Utf8:
    return arrow::utf8();
  case ffi::SchemaDataType::Tag::LargeUtf8:
    return arrow::large_utf8();
  case ffi::SchemaDataType::Tag::Utf8View:
    return arrow::utf8_view();
  case ffi::SchemaDataType::Tag::FixedSizeBinary:
    return arrow::fixed_size_binary(this->value.fixed_size_binary._0);
  default:
    throw std::runtime_error("unreachable");
  }
}

template <>
std::shared_ptr<arrow::RecordBatch>
Convert<ffi::AnyBloxRecordBatch>::into<std::shared_ptr<arrow::RecordBatch>>(
    std::shared_ptr<arrow::Schema> schema) {

  auto ffi_array = reinterpret_cast<ArrowArray *>(&this->value);
  auto root_type = arrow::struct_(schema->fields());
  auto array = std::static_pointer_cast<arrow::StructArray>(
      arrow::ImportArray(ffi_array, root_type).ValueOrDie());

  return arrow::RecordBatch::Make(schema, array->length(), array->fields());
}

template <typename T> Convert<T> convert(T value) { return Convert<T>(value); }

} // namespace ffi
} // namespace anyblox