file(
  GLOB_RECURSE
  SOURCES
  CONFIGURE_DEPENDS
  "include/*.hpp"
  "include/*.h"
  "src/*.cpp"
  "src/*.hpp"
  "src/*.h"
)

add_executable(gameboy-emulator-sdl3 ${SOURCES})

set_target_properties(gameboy-emulator-sdl3
  PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

target_link_libraries(
  gameboy-emulator-sdl3
  PRIVATE
    gb::warnings
    gb::core
    fmt::fmt
    spdlog::spdlog
    SDL3::SDL3
    imgui::imgui
    imgui::imgui-sdl3
)
