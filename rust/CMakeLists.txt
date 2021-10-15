if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

set(RUST_STATIC_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libnetwork.a")

set(RUST_CXX "${CMAKE_CURRENT_BINARY_DIR}/network.cpp")
add_library(network STATIC ${RUST_CXX})
add_custom_command(
    OUTPUT ${RUST_CXX}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/network/src/lib.rs.cc ${RUST_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/network/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/network.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(network ${RUST_STATIC_LIB} pthread dl)

add_test(NAME network_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)


