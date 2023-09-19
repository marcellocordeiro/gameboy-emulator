cmake_minimum_required(VERSION 3.27)

project(gb-ui-sdl2-c)

add_executable(gb-ui-sdl2-c
  "src/main.c"
)

target_include_directories(gb-ui-sdl2-c PRIVATE "../gb-core-c")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_BUILD_FOLDER "../target/debug")
else()
  set(CARGO_BUILD_FOLDER "../target/release")
endif()

target_link_directories(gb-ui-sdl2-c PRIVATE ${CARGO_BUILD_FOLDER})

target_link_libraries(gb-ui-sdl2-c PRIVATE "libgb_core_c.a")

find_package(SDL2 2.0 REQUIRED)
target_link_libraries(gb-ui-sdl2-c PRIVATE SDL2::SDL2)

set_target_properties(gb-ui-sdl2-c
  PROPERTIES
    C_STANDARD 23
    C_STANDARD_REQUIRED YES
    C_EXTENSIONS NO
)

if (UNIX)
  target_compile_options(gb-ui-sdl2-c PRIVATE -Wall -Wextra -pedantic -pedantic-errors)
endif()
