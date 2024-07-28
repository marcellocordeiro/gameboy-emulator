cmake_minimum_required(VERSION 3.30.1)

project("gameboy-emulator")

include("cmake/SetupRustCore.cmake")

add_subdirectory("external/imgui-cmake/imgui")
add_subdirectory("external/imgui-cmake/imgui-backend-sdl2")

add_subdirectory("platform/sdl2-c")
add_subdirectory("platform/sdl2-imgui-cpp")
