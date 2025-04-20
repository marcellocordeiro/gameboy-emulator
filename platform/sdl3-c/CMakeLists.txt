add_executable(gb-sdl3-c
  "src/main.c"
)

target_link_libraries(gb-sdl3-c PRIVATE gb_core_c)

find_package(SDL3 REQUIRED CONFIG REQUIRED COMPONENTS SDL3-shared)
target_link_libraries(gb-sdl3-c PRIVATE SDL3::SDL3)

set_target_properties(gb-sdl3-c
  PROPERTIES
    C_STANDARD 23
    C_STANDARD_REQUIRED YES
    C_EXTENSIONS NO
)

target_link_libraries(gb-sdl3-c PRIVATE project-warnings)
