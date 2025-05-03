file(
  GLOB_RECURSE
  SOURCES
  CONFIGURE_DEPENDS
  "src/*.cpp"
  "src/*.hpp"
  "src/*.h"
)

add_executable(gb-sdl3)
target_sources(gb-sdl3 PRIVATE ${SOURCES})

target_link_libraries(gb-sdl3 PRIVATE gb_core_c)

find_package(SDL3 REQUIRED CONFIG REQUIRED COMPONENTS SDL3-shared)
target_link_libraries(gb-sdl3 PRIVATE SDL3::SDL3)

target_link_libraries(gb-sdl3 PRIVATE imgui imgui-backend-sdl3)

set_target_properties(gb-sdl3
  PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

target_link_libraries(gb-sdl3 PRIVATE project-warnings)
