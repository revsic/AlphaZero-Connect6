cmake_minimum_required(VERSION 3.10)
project(sample_cpp)

set(CMAKE_CXX_STANDARD 17)

add_executable(sample_cpp main.cpp main.cpp)

find_library (
        LIBCONNECT6
        NAMES connect6 libconnect6 # what to look for
        HINTS "${CMAKE_SOURCE_DIR}/../../target/release" # where to look
        NO_DEFAULT_PATH # do not search system default paths
)

# check if we found the library
message(STATUS "LIBCONNECT6: [${LIBCONNECT6}]")

if (NOT LIBCONNECT6)
    message(SEND_ERROR "Did not find libconnect6")
endif()

target_link_libraries(sample_cpp ${LIBCONNECT6})