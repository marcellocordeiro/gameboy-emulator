set(HEADERS
  include/gb/gb.h
)

corrosion_import_crate(
  MANIFEST_PATH "../Cargo.toml"
  CRATES gb-core-ffi
)

add_library(gb::core ALIAS gb_core_ffi)

target_sources(gb_core_ffi
  INTERFACE
    FILE_SET HEADERS
    BASE_DIRS include
    FILES ${HEADERS}
)

set_project_options(gb_core_ffi)
set_compiler_warnings(gb_core_ffi)

# Doesn't work properly with corrosion yet
install(
  TARGETS gb_core_ffi
  # LIBRARY
  FILE_SET HEADERS
)

install(FILES $<TARGET_PROPERTY:gb_core_ffi-static,IMPORTED_LOCATION> DESTINATION lib)
