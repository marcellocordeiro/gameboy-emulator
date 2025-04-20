include(ExternalProject)

# Usage (unused for now. Currently using corrosion)
#setup_rust_core(
#  gb-core # Our CMake target
#  gb-core-c # Rust package
#  "core/gb-core-c" # Header dir
#  "libgb_core_c.a" # Lib
#)

function(setup_rust_core
  TARGET_NAME
  RUST_PACKAGE
  HEADER_DIR
  LIB_NAME
)
  set(RUST_ROOT_PATH ${CMAKE_SOURCE_DIR})
  set(RUST_HEADER_DIR "${RUST_ROOT_PATH}/${HEADER_DIR}")
  set(RUST_TARGET_NAME "${TARGET_NAME}-rust")

  ExternalProject_Add(
    ${RUST_TARGET_NAME}
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build --package ${RUST_PACKAGE}
          COMMAND cargo build --package ${RUST_PACKAGE} --release
    INSTALL_COMMAND ""
    BINARY_DIR ${RUST_ROOT_PATH}
    BUILD_ALWAYS ON
  )

  add_library(${TARGET_NAME} INTERFACE)
  add_dependencies(${TARGET_NAME} ${RUST_TARGET_NAME})

  target_include_directories(${TARGET_NAME}
    INTERFACE
      ${RUST_HEADER_DIR}
  )

  target_link_libraries(${TARGET_NAME}
    INTERFACE
    debug "${RUST_ROOT_PATH}/target/debug/${LIB_NAME}"
    optimized "${RUST_ROOT_PATH}/target/release/${LIB_NAME}"
  )
endfunction()
