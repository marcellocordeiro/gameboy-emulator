file(
  GLOB_RECURSE
  SOURCES
  CONFIGURE_DEPENDS
  "src/*.cpp"
  "src/*.hpp"
  "src/*.h"
)

add_executable(gameboy-emulator-sdl3)
target_sources(gameboy-emulator-sdl3 PRIVATE ${SOURCES})

target_link_libraries(gameboy-emulator-sdl3 PRIVATE gb::core)

find_package(SDL3 CONFIG REQUIRED COMPONENTS SDL3-shared)
target_link_libraries(gameboy-emulator-sdl3 PRIVATE SDL3::SDL3)

find_package(fmt CONFIG REQUIRED)
target_link_libraries(gameboy-emulator-sdl3 PRIVATE fmt::fmt)

find_package(spdlog CONFIG REQUIRED)
target_link_libraries(gameboy-emulator-sdl3 PRIVATE spdlog::spdlog)

target_link_libraries(gameboy-emulator-sdl3 PRIVATE imgui imgui-backend-sdl3)

set_target_properties(gameboy-emulator-sdl3
  PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

target_link_libraries(gameboy-emulator-sdl3 PRIVATE project-warnings)
