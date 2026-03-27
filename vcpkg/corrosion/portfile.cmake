set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)
set(VCPKG_POLICY_DLLS_WITHOUT_LIBS enabled)

vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO corrosion-rs/corrosion
    REF "v${VERSION}"
    SHA512 2b0d1ccafd5472f2938d084995662c586f19cb5cd4ead20fa25e9516d595eb5f756cb2d9abb12dccfa944b52a1f8002c69a1a4d955409c9d194c6d885a6d48ca
    HEAD_REF main
)

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}"
)

vcpkg_cmake_install()
vcpkg_cmake_config_fixup(CONFIG_PATH lib/cmake/Corrosion)
vcpkg_fixup_pkgconfig()

file(REMOVE_RECURSE
    "${CURRENT_PACKAGES_DIR}/debug"
    "${CURRENT_PACKAGES_DIR}/lib"
)

file(INSTALL "${CMAKE_CURRENT_LIST_DIR}/usage" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}")
vcpkg_install_copyright(FILE_LIST "${SOURCE_PATH}/LICENSE")
