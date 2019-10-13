cmake_minimum_required(VERSION 3.14)

project(called-by-go)

add_custom_target(rust-lib ALL
    COMMAND cargo build --target-dir ${CMAKE_CURRENT_BINARY_DIR}/target
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(run-${PROJECT_NAME}
    COMMAND go run main.go
    DEPENDS rust-lib
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/go
)

#set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES
#    ${CMAKE_CURRENT_SOURCE_DIR}/target
#)