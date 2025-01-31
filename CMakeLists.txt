cmake_minimum_required(VERSION 3.21)

# ##############################################################################
# Modules
# ##############################################################################
set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake/modules"
                      ${CMAKE_MODULE_PATH})
set(CMAKE_INCLUDE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake/include/"
                       ${CMAKE_INCLUDE_PATH})

# ##############################################################################
# Include Configurations
# ##############################################################################
message(STATUS "Building Unify with BUILD_TYPE=${CMAKE_BUILD_TYPE}")

include(cmake/include/version.cmake)
include(cmake/include/build_permutations.cmake)

project(
  uic
  LANGUAGES C CXX
  VERSION ${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_REV})

# Set shared version string to use in filenames (deb packages etc.)
set(FILE_NAME_VERSIONING "${CMAKE_PROJECT_VERSION}")
if(VERSION_PATCH)
  string(APPEND FILE_NAME_VERSIONING "-${VERSION_PATCH}")
endif()
set(FILE_NAME_VERSIONING_ARCH
    "${FILE_NAME_VERSIONING}_${CMAKE_SYSTEM_PROCESSOR}")

include(cmake/include/zap.cmake)
include(cmake/include/target_interface_libraries.cmake)
include(cmake/include/compiler_options.cmake)
include(cmake/include/compatible_platform.cmake)
include(cmake/include/unittest.cmake)
include(cmake/include/doxygen.cmake)
include(cmake/include/package-helper.cmake)
include(cmake/include/uic_helper.cmake)

if(BUILD_TESTING)
  include(components/testframework/target_add_unittest.cmake)
endif()

if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
  set(CMAKE_INSTALL_PREFIX
      "/usr"
      CACHE PATH "default install path" FORCE)
endif()

message(STATUS "Installing to ${CMAKE_INSTALL_PREFIX}")

##########
# Python #
##########
find_package(Python3 REQUIRED)

########
# Rust #
########
enable_language(Rust)
set(RUST_MIN_VERSION 1.64)
if(CMAKE_Rust_COMPILER_VERSION VERSION_LESS RUST_MIN_VERSION)
  message(
    FATAL_ERROR
      "Rust version is ${CMAKE_Rust_COMPILER_VERSION}, expect at least ${RUST_MIN_VERSION},
      please update using 'rustup update' or similar,
      see https://www.rust-lang.org/tools/install documentation for further details."
  )
endif()

include(CMakeCargo)

# ##############################################################################
# Global includes
# ##############################################################################
include_directories(include ${CMAKE_CURRENT_BINARY_DIR}/include)

# ##############################################################################
# Subdirectories
# #############################################################################
add_subdirectory(components)
add_subdirectory(applications)
include(cmake/include/package.cmake)
include(cmake/include/version_file.cmake)
