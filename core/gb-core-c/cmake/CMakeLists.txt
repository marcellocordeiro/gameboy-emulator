CPMAddPackage("gh:corrosion-rs/corrosion#v0.5.2")

corrosion_import_crate(
  MANIFEST_PATH "../Cargo.toml"
  CRATES "gb-core-c"
)

target_sources(gb_core_c INTERFACE
  FILE_SET HEADERS
  BASE_DIRS "include"
  FILES
  "include/gb/gb.h"
)

add_library(gb::core ALIAS gb_core_c)
