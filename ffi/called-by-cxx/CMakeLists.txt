cmake_minimum_required(VERSION 3.13)

project(called-by-cxx CXX)

set(CMAKE_CXX_STANDARD 17)

add_custom_target(rust-lib ALL
    cargo build --release --target-dir=${PROJECT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_subdirectory(cxx)

set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)