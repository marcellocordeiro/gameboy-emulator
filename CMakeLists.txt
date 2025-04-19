cmake_minimum_required(VERSION 3.30.5)

project(gameboy-emulator)

include("cmake/SetupRustCore.cmake")

add_subdirectory("external/imgui-cmake/imgui")
add_subdirectory("external/imgui-cmake/imgui-backend-sdl3")

add_subdirectory("platform/sdl3-c")
add_subdirectory("platform/sdl3-imgui-cpp")
