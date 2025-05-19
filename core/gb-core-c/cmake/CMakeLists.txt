set(HEADERS
  include/gb/gb.h
)

corrosion_import_crate(
  MANIFEST_PATH "../Cargo.toml"
  CRATES gb-core-c
)

add_library(gb::core ALIAS gb_core_c)

target_sources(gb_core_c
  INTERFACE
    FILE_SET HEADERS
    BASE_DIRS include
    FILES ${HEADERS}
)

set_project_options(gb_core_c)
set_compiler_warnings(gb_core_c)

# Doesn't work properly with corrosion yet
install(
  TARGETS gb_core_c
  # LIBRARY
  FILE_SET HEADERS
)

install(FILES $<TARGET_PROPERTY:gb_core_c-static,IMPORTED_LOCATION> DESTINATION lib)
