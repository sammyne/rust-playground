cmake_minimum_required(VERSION 3.13)

project(called-by-c C)

add_custom_target(rust-lib ALL
    cargo build --release --target-dir=${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_subdirectory(c)

set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/c/include
)