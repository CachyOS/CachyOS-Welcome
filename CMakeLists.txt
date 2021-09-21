cmake_minimum_required(VERSION 3.14)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

##
## PROJECT
## name and version
##
project(cachyos-hello CXX)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE RelWithDebInfo)
endif()
message(STATUS "BUILD: ${CMAKE_BUILD_TYPE}")


##
## INCLUDE
##
include(GNUInstallDirs)
include(CompilerWarnings)
include(EnableCcache)
include(ClangTidy)
include(FetchContent)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtkmm-3.0)

FetchContent_Declare(fmt
  GIT_REPOSITORY "https://github.com/fmtlib/fmt.git"
  GIT_TAG "2742611cad4aee6b1a5638bd1ebf132908f4a3d9"
)
FetchContent_MakeAvailable(fmt)

##
## CONFIGURATION
##
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -flto")

if(UNIX AND CMAKE_GENERATOR STREQUAL "Ninja")
  if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    set(CMAKE_CXX_FLAGS "-fcolor-diagnostics ${CMAKE_CXX_FLAGS}")
    set(CMAKE_C_FLAGS "-fcolor-diagnostics ${CMAKE_C_FLAGS}")
  endif()
  if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    set(CMAKE_CXX_FLAGS "-fdiagnostics-color ${CMAKE_CXX_FLAGS}")
    set(CMAKE_C_FLAGS "-fdiagnostics-color ${CMAKE_C_FLAGS}")
  endif()
endif()

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -fwhole-program")
endif()

add_library(project_warnings INTERFACE)
set_project_warnings(project_warnings)

##
## Target
##
add_executable(${PROJECT_NAME}
    src/hello.cpp src/hello.hpp
    src/main.cpp
    )

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)

include_directories(${CMAKE_SOURCE_DIR}/src ${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})
add_definitions(${GTK3_CFLAGS_OTHER})

target_link_libraries(${PROJECT_NAME} ${GTK3_LIBRARIES} fmt::fmt)

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
