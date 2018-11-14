cmake_minimum_required(VERSION 3.10)
project(sample_cpp)

set(CMAKE_CXX_STANDARD 17)
set(CPPBIND ${CMAKE_SOURCE_DIR}/../../cppbind)
set(LIBCONNECT6_PATH ${CMAKE_SOURCE_DIR}/../../target/release)

if(WIN32)
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/connect6.dll.lib)
elseif(APPLE)
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/libconnect6.a)
else()
    set(LIBCONNECT6 ${LIBCONNECT6_PATH/libconnect6.so})
endif()

add_executable(sample_cpp main.cpp)

include_directories(sample_cpp PRIVATE ${CPPBIND})
target_link_libraries(sample_cpp ${LIBCONNECT6})

if(APPLE)
    target_link_libraries(sample_cpp "-framework Security")
elseif(UNIX)
    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)
    target_link_libraries(sample_cpp Threads::Threads)
endif()
