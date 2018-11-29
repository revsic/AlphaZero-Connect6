cmake_minimum_required(VERSION 3.10)
project(cpp_weighted)

set(CMAKE_CXX_STANDARD 17)
add_executable(cpp_weighted main.cpp)

set(LIBCONNECT6_PATH ${CMAKE_SOURCE_DIR}/../../../Connect6)

if(WIN32)
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/target/release/cppconnect6.lib)
else()
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/target/release/libcppconnect6.a)
endif()

target_link_libraries(cpp_weighted ${LIBCONNECT6})
target_include_directories(cpp_weighted PRIVATE ${LIBCONNECT6_PATH}/cppbind)

set(DIST ${CMAKE_SOURCE_DIR}/../../../dist)
set(TF_SOURCE ${DIST}/includes)
set(TF_EXTERNAL ${TF_SOURCE}/external)

target_link_libraries(cpp_weighted ${DIST}/libtensorflow.so)
target_include_directories(cpp_weighted PRIVATE ${TF_SOURCE})
target_include_directories(cpp_weighted PRIVATE ${TF_EXTERNAL}/protobuf_archive/src)
target_include_directories(cpp_weighted PRIVATE ${TF_EXTERNAL}/eigen_archive)
target_include_directories(cpp_weighted PRIVATE ${TF_EXTERNAL}/com_google_absl)

if(APPLE)
    target_link_libraries(cpp_weighted "-framework Security")
elseif(UNIX)
    target_link_libraries(cpp_weighted dl)
    target_link_libraries(cpp_weighted pthread)
else()
    target_link_libraries(cpp_weighted ws2_32)
    target_link_libraries(cpp_weighted userenv)
endif()
