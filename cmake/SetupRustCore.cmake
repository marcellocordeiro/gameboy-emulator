if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_BUILD_FOLDER "../../target/debug")
else()
  set(CARGO_BUILD_FOLDER "../../target/release")
endif()

set(GB_HEADER_DIR "../../core/gb-core-c")
set(GB_LIB_NAME "libgb_core_c.a")
