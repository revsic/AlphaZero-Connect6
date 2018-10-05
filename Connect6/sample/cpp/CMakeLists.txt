cmake_minimum_required(VERSION 3.10)
project(sample_cpp)

set(CMAKE_CXX_STANDARD 17)
set(LIBCONNECT6 ${CMAKE_SOURCE_DIR}/../../target/release/libconnect6.a)

add_executable(sample_cpp main.cpp)

target_link_libraries(sample_cpp ${LIBCONNECT6})
