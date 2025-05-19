# include("cmake/get_cpm.cmake")
#
# CPMAddPackage("gh:corrosion-rs/corrosion#v0.5.2")
#
# CPMAddPackage(
#   URI "gh:fmtlib/fmt#11.2.0"
# )
#
# CPMAddPackage(
#   URI "gh:gabime/spdlog#v1.15.3"
#   OPTIONS
#     "SPDLOG_FMT_EXTERNAL ON"
#     "SPDLOG_BUILD_SHARED OFF"
# )
#
# CPMAddPackage(
#   URI "gh:libsdl-org/SDL#release-3.2.14"
# )
#
# CPMAddPackage(
#   URI "gh:catchorg/Catch2#v3.8.1"
# )
#
# list(APPEND CMAKE_MODULE_PATH ${Catch2_SOURCE_DIR}/extras)

find_package(fmt CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(SDL3 CONFIG REQUIRED)
find_package(Corrosion CONFIG REQUIRED)

add_subdirectory(external/imgui-cmake EXCLUDE_FROM_ALL SYSTEM)
