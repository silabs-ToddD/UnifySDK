add_library(zpc_scripts INTERFACE)
target_include_directories(zpc_scripts INTERFACE
  $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}>)

configure_file(${CMAKE_CURRENT_LIST_DIR}/zpc/script_defines.h.in
${CMAKE_CURRENT_BINARY_DIR}/script_defines.h @ONLY)

configure_file(${CMAKE_CURRENT_LIST_DIR}/zpc/node_identify_rpi4_led.sh
  ${CMAKE_CURRENT_BINARY_DIR}/node_identify_rpi4_led.sh @ONLY)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/node_identify_rpi4_led.sh DESTINATION
        ${CMAKE_INSTALL_PREFIX}/zpc COMPONENT uic-zpc)

