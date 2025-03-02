cmake_minimum_required(VERSION 3.12)

project(
  AnyBloxCpp
  VERSION 0.10.0
  LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_DEBUG_POSTFIX d)
include(GNUInstallDirs)

set(OPENSSL_USE_STATIC_LIBS TRUE)
find_package(OpenSSL REQUIRED)

find_library(LIBANYBLOX_FFI "libanyblox_ffi.a" PATHS ${CMAKE_SOURCE_DIR}/lib)
find_library(LIBANYBLOX_FFI_DEBUG "libanyblox_ffid.a" PATHS ${CMAKE_SOURCE_DIR}/lib)
find_package(Arrow REQUIRED)

add_library(AnyBloxCpp SHARED src/anyblox.cpp src/sysutil.cpp)

target_include_directories(
  AnyBloxCpp
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}/anyblox>
)
target_include_directories(
  AnyBloxCpp
  PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}
)
target_link_libraries(AnyBloxCpp PRIVATE optimized "${LIBANYBLOX_FFI}" debug "${LIBANYBLOX_FFI_DEBUG}")
target_link_libraries(AnyBloxCpp PRIVATE OpenSSL::Crypto OpenSSL::SSL)
target_link_libraries(AnyBloxCpp PUBLIC Arrow::arrow_shared)

set_target_properties(AnyBloxCpp PROPERTIES DEBUG_POSTFIX ${CMAKE_DEBUG_POSTFIX})
set_target_properties(AnyBloxCpp PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(AnyBloxCpp PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})

add_executable(AnyBloxCppExampleTrunc8 src/example_trunc8.cpp)
add_executable(AnyBloxCppExampleTaxpayer src/example_taxpayer.cpp)

target_include_directories(AnyBloxCppExampleTrunc8 PRIVATE include)
target_include_directories(AnyBloxCppExampleTaxpayer PRIVATE include)
target_link_libraries(AnyBloxCppExampleTrunc8 PRIVATE AnyBloxCpp)
target_link_libraries(AnyBloxCppExampleTaxpayer PRIVATE AnyBloxCpp)

include(CMakePackageConfigHelpers)

install(
  TARGETS AnyBloxCpp
  EXPORT AnyBloxCppTargets
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/anyblox
)
install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/anyblox)

write_basic_package_version_file(
  "${CMAKE_CURRENT_BINARY_DIR}/AnyBloxCppConfigVersion.cmake"
  VERSION ${PROJECT_VERSION}
  COMPATIBILITY ExactVersion
)
configure_package_config_file(
  ${CMAKE_CURRENT_LIST_DIR}/cmake/AnyBloxCppConfig.cmake.in
  ${CMAKE_CURRENT_BINARY_DIR}/AnyBloxCppConfig.cmake
  INSTALL_DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/cmake/anybloxcpp
)

install(
  EXPORT AnyBloxCppTargets
  FILE AnyBloxCppTargets.cmake
  NAMESPACE AnyBlox::
  DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/cmake/anybloxcpp
)

install(
  FILES
    "${CMAKE_CURRENT_BINARY_DIR}/AnyBloxCppConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/AnyBloxCppConfigVersion.cmake"
  DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/cmake/anybloxcpp
)
