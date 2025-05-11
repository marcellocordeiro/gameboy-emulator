set(SOURCES
  src/app.cpp
  src/app.hpp
  src/main.cpp
  src/imgui/imgui.hpp
  src/sdl/sdl.hpp
  src/sdl/sdl_context.hpp
  src/sdl/sdl_error.hpp
  src/sdl/sdl_include.hpp
  src/sdl/sdl_renderer.hpp
  src/sdl/sdl_texture.hpp
  src/sdl/sdl_window.hpp
  src/utils/files.hpp
  src/utils/scaling.hpp
  src/utils/vec2.hpp
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
    global::warnings
    gb::core
    fmt::fmt
    spdlog::spdlog
    SDL3::SDL3
    imgui::imgui
    imgui::imgui-sdl3
)

install(
  TARGETS gameboy-emulator-sdl3
)
