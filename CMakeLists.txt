cmake_minimum_required(VERSION 3.11)

project(
    network
    VERSION 1.0
    DESCRIPTION "Rust Networking in C++"
    LANGUAGES CXX)

add_subdirectory(rust)

include_directories(${CMAKE_BINARY_DIR}/rust)

add_executable(main main.cc)

target_link_libraries(main network)
