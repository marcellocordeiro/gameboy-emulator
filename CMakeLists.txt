cmake_minimum_required(VERSION 3.30.5...4.0.1)

project(gameboy-emulator)

include("cmake/CompilerWarnings.cmake")
include("cmake/CPM.cmake")
include("cmake/StandardProjectSettings.cmake")

add_library(project-warnings INTERFACE)
set_project_warnings(project-warnings)

add_subdirectory("core/gb-core-c/cmake")

add_subdirectory("external/imgui-cmake/imgui")
add_subdirectory("external/imgui-cmake/imgui-backend-sdl3")

add_subdirectory("platform/sdl3-imgui-cpp")

# Workaround to make clangd find `compile_commands.json`
if(CLANGD_CONFIG_WORKAROUND)
  file(WRITE ${CMAKE_SOURCE_DIR}/.clangd
"CompileFlags:
  CompilationDatabase: ${CMAKE_BINARY_DIR}
"
  )
endif()
