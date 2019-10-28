cmake_minimum_required(VERSION 3.14)

set(program hello-world)

set(srcs ${CMAKE_CURRENT_SOURCE_DIR}/main.c)
set(LDFLAGS "-L${PROJECT_BINARY_DIR}/release -lhelloworld")

add_executable(${program} ${srcs})
add_dependencies(${program} rust-lib)
target_link_libraries(${program} ${LDFLAGS})

add_custom_target(run-${program}
    COMMAND ${CMAKE_CURRENT_BINARY_DIR}/${program}
    DEPENDS ${program}
    COMMENT "running ${program}"
)