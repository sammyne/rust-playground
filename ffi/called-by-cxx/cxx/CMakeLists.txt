set(program hello-world)

set(srcs ${CMAKE_CURRENT_SOURCE_DIR}/main.cc)
set(LDFLAGS "-L${PROJECT_BINARY_DIR}/release -lhelloworld")

add_executable(${program} ${srcs})
add_dependencies(${program} rust-lib)
target_link_libraries(${program} ${LDFLAGS} pthread dl)
target_include_directories(${program} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)

add_custom_target(run-${program}
    COMMAND ${CMAKE_CURRENT_BINARY_DIR}/${program}
    DEPENDS ${program}
    COMMENT "running ${program}"
)