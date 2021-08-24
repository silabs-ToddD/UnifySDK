
set(CPACK_COMPONENTS_GROUPING "IGNORE")
set(CPACK_DEB_COMPONENT_INSTALL ON)
set(CPACK_DEBIAN_ENABLE_COMPONENT_DEPENDS ON)
set(CPACK_DEB_PACKAGE_COMPONENT ON)

set(CPACK_PACKAGE_FILE_NAME
  "${CMAKE_PROJECT_NAME}_${CMAKE_PROJECT_VERSION_MAJOR}.${CMAKE_PROJECT_VERSION_MINOR}.${CMAKE_PROJECT_VERSION_PATCH}-${VERSION_PATCH}_${CMAKE_SYSTEM_PROCESSOR}"
)
# Common CPACK configuration
set(CPACK_PACKAGING_INSTALL_PREFIX "")
# Generate source tarball
set(CPACK_SOURCE_GENERATOR "ZIP")
set(CPACK_SOURCE_IGNORE_FILES
    "/build.*/"
    "GeckoSDK/"
    "Jenkinsfile"
    "\\\\.git*"
    "\\\\.pre-commit-config.yaml"
    "sonar-project\\\\.properties"
    "scripts/ci"
    "scripts/internal"
    ".*\\\\.internal\\\\.md")

set(CPACK_SOURCE_PACKAGE_FILE_NAME
  "${CMAKE_PROJECT_NAME}-${CMAKE_PROJECT_VERSION_MAJOR}.${CMAKE_PROJECT_VERSION_MINOR}.${CMAKE_PROJECT_VERSION_PATCH}-${VERSION_PATCH}-Source"
)

# Generate Debian package
if(NOT APPLE)
    set(CPACK_GENERATOR "DEB")
    set(CPACK_DEBIAN_PACKAGE_MAINTAINER "Silicon Labs")

    # Function arguments
    # 1. PKG_NAME      component package Name
    # 2. PKG_FILE_NAME Debian package filename for the component
    # 3. PKG_DEPNDS    Component debian package depenedencies.
    # 4. PKG_EXTRA     Component debian package Control extra info
    # See https://cmake.org/cmake/help/v3.6/module/CPackDeb.html for more info

    function (add_component_to_uic PKG_NAME PKG_FILE_NAME PKG_DEPNDS PKG_EXTRA)
      string(TOUPPER ${PKG_NAME} PKG_NAME_UPPER)
      #Removing spaces generated by identation below
      string(REGEX REPLACE " " "" PKG_DEPNDS "${PKG_DEPNDS}")
      set(CPACK_DEBIAN_${PKG_NAME_UPPER}_PACKAGE_DEPENDS ${PKG_DEPNDS}
          PARENT_SCOPE)
      string(CONCAT CPACK_DEBIAN_${PKG_NAME_UPPER}_FILE_NAME
             "${PKG_FILE_NAME}_"
             "${CMAKE_PROJECT_VERSION_MAJOR}."
             "${CMAKE_PROJECT_VERSION_MINOR}."
             "${CMAKE_PROJECT_VERSION_PATCH}_"
             "${CMAKE_SYSTEM_PROCESSOR}.deb"
             )
      set(CPACK_DEBIAN_${PKG_NAME_UPPER}_FILE_NAME
          ${CPACK_DEBIAN_${PKG_NAME_UPPER}_FILE_NAME}
          PARENT_SCOPE)
      set(CPACK_DEBIAN_${PKG_NAME_UPPER}_PACKAGE_NAME ${PKG_NAME} PARENT_SCOPE)
      set(CPACK_DEBIAN_${PKG_NAME_UPPER}_DESCRIPTION
        "UIC component ${PKG_NAME}" PARENT_SCOPE)

      #Removing spaces generated by identation below
      string(REGEX REPLACE " " "" PKG_EXTRA "${PKG_EXTRA}")
      set(CPACK_DEBIAN_${PKG_NAME_UPPER}_PACKAGE_CONTROL_EXTRA ${PKG_EXTRA} PARENT_SCOPE)

      install(FILES "${CMAKE_SOURCE_DIR}/copyright" DESTINATION share/doc/${PKG_NAME} COMPONENT ${PKG_NAME})

    endfunction()

    # Lib uic will always be build
    set(CPACK_COMPONENTS_ALL "libuic")
    # If all applications is build create uic-all as well

    add_component_to_uic(
      libuic
      "libuic"
      "libboost-program-options1.67.0,\
       libedit2, libsqlite3-0, libmosquitto1,\
       libyaml-cpp0.6, libboost-log1.67.0"
      "")

    if(BUILD_ZPC)
      add_component_to_uic(
        uic-zpc                                           # Package Name
        "${CMAKE_PROJECT_NAME}-zpc"                   # Package Debian filename
        "libuic"                                      # Package Depends on
        "${CMAKE_BINARY_DIR}/applications/zpc/debconf/config;\
        ${CMAKE_BINARY_DIR}/applications/zpc/debconf/templates;\
        ${CMAKE_BINARY_DIR}/applications/zpc/debconf/postinst"
        )
      list(APPEND CPACK_COMPONENTS_ALL "uic-zpc")
    endif()

    if(BUILD_DEV_CLI)
      add_component_to_uic(
        uic-dev-cli
        "${CMAKE_PROJECT_NAME}-dev-cli"
        "libboost-log1.67.0"
        ""
        )
      list(APPEND CPACK_COMPONENTS_ALL "uic-dev-cli")
    endif()

    if(BUILD_DEV_GUI)
      add_component_to_uic(
        uic-dev-gui
        "${CMAKE_PROJECT_NAME}-dev-gui"
        ""
        "${CMAKE_BINARY_DIR}/applications/dev_ui/dev_gui/debconf/postinst;\
        ${CMAKE_BINARY_DIR}/applications/dev_ui/dev_gui/debconf/prerm;\
        ${CMAKE_BINARY_DIR}/applications/dev_ui/dev_gui/debconf/postrm"
      )
      list(APPEND CPACK_COMPONENTS_ALL "uic-dev-gui")
    endif()

    if(BUILD_ZIGPC)
      add_component_to_uic(
        uic-zigpc
        "${CMAKE_PROJECT_NAME}-zigpc"
        "libuic"
        ""
        )
      list(APPEND CPACK_COMPONENTS_ALL "uic-zigpc")
    endif()
    list(JOIN CPACK_COMPONENTS_ALL "," COMPONENTS-ALL)
    add_component_to_uic(
      uic-all
      "${CMAKE_PROJECT_NAME}-all"
      ${COMPONENTS-ALL}
      ""
      )

    list(APPEND CPACK_COMPONENTS_ALL uic-all)

    message(STATUS "Components of UIC which will have deb packages"
                   ": ${CPACK_COMPONENTS_ALL}")

endif()

include(CPack)
add_custom_target(deb DEPENDS package)
