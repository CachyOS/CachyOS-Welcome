cmake_minimum_required(VERSION 3.14)

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
include(FetchContent)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtkmm-3.0)

FetchContent_Declare(fmt
  GIT_REPOSITORY "https://github.com/fmtlib/fmt.git"
  GIT_TAG "d9fd695ac737f84f7de2d0a2aa346b25efb9afbf"
)
FetchContent_MakeAvailable(fmt)

##
## CONFIGURATION
##
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

set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -flto")

##
## Target
##
add_executable(${PROJECT_NAME}
    src/main.cc
    )

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_17)

include_directories(${CMAKE_SOURCE_DIR}/src ${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})
add_definitions(${GTK3_CFLAGS_OTHER})

target_link_libraries(${PROJECT_NAME} ${GTK3_LIBRARIES} fmt::fmt)
