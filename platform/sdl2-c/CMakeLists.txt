cmake_minimum_required(VERSION 3.27)

project(gameboy-emulator)

add_executable(gameboy-emulator
  "src/main.c"
)

target_include_directories(gameboy-emulator PRIVATE "../../core/gb-core-c")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_BUILD_FOLDER "../../target/debug")
else()
  set(CARGO_BUILD_FOLDER "../../target/release")
endif()

target_link_directories(gameboy-emulator PRIVATE ${CARGO_BUILD_FOLDER})

target_link_libraries(gameboy-emulator PRIVATE "libgb_core_c.a")

find_package(SDL2 2.0 REQUIRED)
target_link_libraries(gameboy-emulator PRIVATE SDL2::SDL2)

set_target_properties(gameboy-emulator
  PROPERTIES
    C_STANDARD 23
    C_STANDARD_REQUIRED YES
    C_EXTENSIONS NO
)

if (UNIX)
  target_compile_options(gameboy-emulator PRIVATE -Wall -Wextra -pedantic -pedantic-errors)
endif()
