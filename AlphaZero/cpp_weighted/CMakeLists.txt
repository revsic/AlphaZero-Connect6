cmake_minimum_required(VERSION 3.10)
project(cpp_weighted)

set(CMAKE_CXX_STANDARD 17)
add_executable(cpp_weighted main.cpp)

set(ROOT_DIR ${CMAKE_SOURCE_DIR}/../..)
set(CONNECT6 ${ROOT_DIR}/Connect6)
set(EXTERNAL ${ROOT_DIR}/External)

# Link LibTorch
set(CMAKE_PREFIX_PATH ${EXTERNAL}/libtorch)

find_package(Torch REQUIRED)
target_link_libraries(cpp_weighted ${TORCH_LIBRARIES})

# link cxxopts
target_include_directories(cpp_weighted PRIVATE ${EXTERNAL}/cxxopts/include)

# link json
target_include_directories(cpp_weighted PRIVATE ${EXTERNAL}/json/single_include)

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
