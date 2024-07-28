cmake_minimum_required(VERSION 3.30.1)

project(gb-sdl2-cpp)

add_executable(${PROJECT_NAME}
  "src/main.cpp"
)

target_include_directories(${PROJECT_NAME} PRIVATE ${GB_HEADER_DIR})
target_link_directories(${PROJECT_NAME} PRIVATE ${CARGO_BUILD_FOLDER})
target_link_libraries(${PROJECT_NAME} PRIVATE ${GB_LIB_NAME})

find_package(SDL2 2.0 REQUIRED)
target_link_libraries(${PROJECT_NAME} PRIVATE SDL2::SDL2)

target_link_libraries(${PROJECT_NAME} PRIVATE imgui)
target_link_libraries(${PROJECT_NAME} PRIVATE imgui-backend-sdl2)

set_target_properties(${PROJECT_NAME}
  PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

if (UNIX)
  target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -pedantic -pedantic-errors)
endif()
