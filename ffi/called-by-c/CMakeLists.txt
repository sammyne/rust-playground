cmake_minimum_required(VERSION 3.14)

project(called-by-c)

add_custom_target(rust-lib ALL
    cargo build --release --target-dir=${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set(srcs ${CMAKE_CURRENT_SOURCE_DIR}/src/main.c)
set(LDFLAGS "-L${CMAKE_CURRENT_BINARY_DIR}/release -lhelloworld")

add_executable(hello-world ${srcs})
add_dependencies(hello-world rust-lib)
target_link_libraries(hello-world ${LDFLAGS})