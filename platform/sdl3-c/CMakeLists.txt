cmake_minimum_required(VERSION 4.0.0)

project(gb-sdl3-c)

add_executable(${PROJECT_NAME}
  "src/main.c"
)

target_include_directories(${PROJECT_NAME} PRIVATE ${GB_HEADER_DIR})
target_link_directories(${PROJECT_NAME} PRIVATE ${CARGO_BUILD_FOLDER})
target_link_libraries(${PROJECT_NAME} PRIVATE ${GB_LIB_NAME})

find_package(SDL3 REQUIRED CONFIG REQUIRED COMPONENTS SDL3-shared)
target_link_libraries(${PROJECT_NAME} PRIVATE SDL3::SDL3)

set_target_properties(${PROJECT_NAME}
  PROPERTIES
    C_STANDARD 23
    C_STANDARD_REQUIRED YES
    C_EXTENSIONS NO
)

if (UNIX)
  target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -pedantic -pedantic-errors)
endif()
