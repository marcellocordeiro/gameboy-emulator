CPMAddPackage("gh:corrosion-rs/corrosion#v0.5.1")

corrosion_import_crate(
  MANIFEST_PATH "../Cargo.toml"
  CRATES "gb-core-c"
  NO_LINKER_OVERRIDE
)

target_sources(gb_core_c INTERFACE
  FILE_SET HEADERS
  BASE_DIRS "include"
  FILES
  "include/gb/gb.h"
)
