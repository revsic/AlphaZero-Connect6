cmake_minimum_required(VERSION 3.10)
project(test_cppbind)

set(CMAKE_CXX_STANDARD 17)
set(LIBCONNECT6_PATH ${CMAKE_SOURCE_DIR}/../../target/release)

if(WIN32)
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/cppconnect6.lib)
else()
    set(LIBCONNECT6 ${LIBCONNECT6_PATH}/libcppconnect6.a)
endif()

add_executable(test_cppbind main.cpp)
target_link_libraries(test_cppbind ${LIBCONNECT6})

if(APPLE)
    target_link_libraries(test_cppbind "-framework Security")
elseif(UNIX)
    target_link_libraries(test_cppbind dl)
    target_link_libraries(test_cppbind pthread)
else()
    target_link_libraries(test_cppbind ws2_32)
    target_link_libraries(test_cppbind userenv)
endif()
