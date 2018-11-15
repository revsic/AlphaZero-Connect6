cmake_minimum_required(VERSION 3.10)
project(sample_cpp)

set(CMAKE_CXX_STANDARD 17)
set(CPPBIND ${CMAKE_SOURCE_DIR}/../../cppbind)
set(LIBCONNECT6_PATH ${CMAKE_SOURCE_DIR}/../../target/release)

if(WIN32)
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/cppconnect6.lib)
else()
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/libcppconnect6.a)
endif()

add_executable(sample_cpp main.cpp)

include_directories(sample_cpp PRIVATE ${CPPBIND})
target_link_libraries(sample_cpp ${LIBCONNECT6})

if(APPLE)
    target_link_libraries(sample_cpp "-framework Security")
elseif(UNIX)
    target_link_libraries(sample_cpp dl)
    target_link_libraries(sample_cpp pthread)
else()
    target_link_libraries(sample_cpp ws2_32)
    target_link_libraries(sample_cpp userenv)
endif()
