cmake_minimum_required(VERSION 3.30.1)

project(gameboy-emulator-sdl2-imgui)

add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/../../external/imgui-cmake/imgui" "imgui-build")
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/../../external/imgui-cmake/imgui-backend-sdl2" "imgui-build/backends/sdl2")

add_executable(gameboy-emulator-sdl2-imgui
  "src/main.cpp"
)

target_include_directories(gameboy-emulator-sdl2-imgui PRIVATE "../../core/gb-core-c")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_BUILD_FOLDER "../../target/debug")
else()
  set(CARGO_BUILD_FOLDER "../../target/release")
endif()

target_link_directories(gameboy-emulator-sdl2-imgui PRIVATE ${CARGO_BUILD_FOLDER})

target_link_libraries(gameboy-emulator-sdl2-imgui PRIVATE "libgb_core_c.a")

find_package(SDL2 2.0 REQUIRED)
target_link_libraries(gameboy-emulator-sdl2-imgui PRIVATE SDL2::SDL2)

target_link_libraries(gameboy-emulator-sdl2-imgui PRIVATE imgui)
target_link_libraries(gameboy-emulator-sdl2-imgui PRIVATE imgui-backend-sdl2)

set_target_properties(gameboy-emulator-sdl2-imgui
  PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

if (UNIX)
  target_compile_options(gameboy-emulator-sdl2-imgui PRIVATE -Wall -Wextra -pedantic -pedantic-errors)
endif()
