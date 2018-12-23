cmake_minimum_required(VERSION 3.10)
project(cpp_weighted)

set(CMAKE_CXX_STANDARD 14)
add_executable(cpp_weighted main.cpp)

set(ROOT_DIR ${CMAKE_SOURCE_DIR}/../../..)
set(CONNECT6 ${ROOT_DIR}/Connect6)
set(TORCH ${ROOT_DIR}/External/libtorch)

# Link LibTorch
set(CMAKE_PREFIX_PATH ${TORCH})

find_package(Torch REQUIRED)
target_link_libraries(cpp_weighted ${TORCH_LIBRARIES})

# Link LibConnect6
if(WIN32)
    set(LIBCONNECT6 ${CONNECT6}/target/release/cppconnect6.lib)
else()
    set(LIBCONNECT6 ${CONNECT6}/target/release/libcppconnect6.a)
endif()

target_link_libraries(cpp_weighted ${LIBCONNECT6})
target_include_directories(cpp_weighted PRIVATE ${CONNECT6}/cppbind)

if(APPLE)
    target_link_libraries(cpp_weighted "-framework Security")
elseif(UNIX)
    target_link_libraries(cpp_weighted dl)
    target_link_libraries(cpp_weighted pthread)
else()
    target_link_libraries(cpp_weighted ws2_32)
    target_link_libraries(cpp_weighted userenv)
endif()
