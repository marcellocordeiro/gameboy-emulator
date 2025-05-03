file(
  GLOB_RECURSE
  SOURCES
  CONFIGURE_DEPENDS
  "src/*.cpp"
  "src/*.hpp"
  "src/*.h"
)

add_executable(gb-sdl3-cpp)
target_sources(gb-sdl3-cpp PRIVATE ${SOURCES})

target_link_libraries(gb-sdl3-cpp PRIVATE gb_core_c)

find_package(SDL3 REQUIRED CONFIG REQUIRED COMPONENTS SDL3-shared)
target_link_libraries(gb-sdl3-cpp PRIVATE SDL3::SDL3)

target_link_libraries(gb-sdl3-cpp PRIVATE imgui imgui-backend-sdl3)

set_target_properties(gb-sdl3-cpp
  PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

target_link_libraries(gb-sdl3-cpp PRIVATE project-warnings)
