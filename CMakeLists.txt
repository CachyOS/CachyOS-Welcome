cmake_minimum_required(VERSION 3.15)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

##
## PROJECT
## name and version
##
project(cachyos-hello CXX)

##
## INCLUDE
##
include(GNUInstallDirs)
include(StandardProjectSettings)
include(CompilerWarnings)
include(EnableCcache)
include(ClangTidy)
include(FetchContent)

find_package(PkgConfig REQUIRED)
pkg_check_modules(
  GTKMM
  REQUIRED
  IMPORTED_TARGET
  gtkmm-4.0)

FetchContent_Declare(fmt
  GIT_REPOSITORY "https://github.com/fmtlib/fmt.git"
  GIT_TAG "a44716f58e943905d1357160b98cae2618d053cf"
)
FetchContent_MakeAvailable(fmt)

##
## CONFIGURATION
##
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wl,--export-dynamic")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -flto")

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -fwhole-program")
endif()

# Link this 'library' to set the c++ standard / compile-time options requested
add_library(project_options INTERFACE)
target_compile_features(project_options INTERFACE cxx_std_17)

##
## Target
##
add_executable(${PROJECT_NAME}
    src/hello.cpp src/hello.hpp
    src/main.cpp
    )

# Link this 'library' to use the warnings specified in CompilerWarnings.cmake
add_library(project_warnings INTERFACE)
set_project_warnings(project_warnings)

include_directories(${CMAKE_SOURCE_DIR}/src ${GTK3_INCLUDE_DIRS})

target_link_libraries(${PROJECT_NAME} PRIVATE project_warnings project_options PkgConfig::GTKMM fmt::fmt)

option(ENABLE_UNITY "Enable Unity builds of projects" OFF)
if(ENABLE_UNITY)
  # Add for any project you want to apply unity builds for
  set_target_properties(${PROJECT_NAME} PROPERTIES UNITY_BUILD ON)
endif()

install(
    TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

install(
    FILES ${CMAKE_SOURCE_DIR}/cachyos-hello.desktop
    DESTINATION ${CMAKE_INSTALL_DATADIR}/applications
)

install(
    DIRECTORY ${CMAKE_SOURCE_DIR}/data
    DESTINATION ${CMAKE_INSTALL_DATADIR}/${PROJECT_NAME}
)

install(
    DIRECTORY ${CMAKE_SOURCE_DIR}/ui
    DESTINATION ${CMAKE_INSTALL_DATADIR}/${PROJECT_NAME}
)


# uninstall
add_custom_target(uninstall
    COMMAND cat ${PROJECT_BINARY_DIR}/install_manifest.txt | xargs rm
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)
