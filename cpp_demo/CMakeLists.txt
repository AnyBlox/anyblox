cmake_minimum_required(VERSION 3.12)

project(cpp_demo)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(anyblox)

find_library(LIBANYBLOX libanyblox.a PATHS ${CMAKE_SOURCE_DIR}/anyblox)
find_package(Arrow REQUIRED)

add_executable(cpp_demo src/main.cpp)

target_link_libraries(cpp_demo PRIVATE "${LIBANYBLOX}")
target_link_libraries(cpp_demo PRIVATE Arrow::arrow_shared)