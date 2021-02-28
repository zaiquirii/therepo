cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
SET(CONAN_DISABLE_CHECK_COMPILER true)
include(./build/conanbuildinfo.cmake)
conan_basic_setup()
SET_PROPERTY(GLOBAL PROPERTY TARGET_SUPPORTS_SHARED_LIBS TRUE)
find_package(SDL2 REQUIRED COMPONENTS main)

project(lightbikes VERSION 0.1)

add_subdirectory(../yage ${PROJECT_BINARY_DIR}/build_yage)

file(GLOB_RECURSE LIGHTBIKE_SRCS ${PROJECT_SOURCE_DIR}/src/lightbikes/*.cpp)
file(GLOB_RECURSE LIGHTBIKE_SRCS_TEST ${PROJECT_SOURCE_DIR}/src/lightbikes/*.test.cpp)

add_executable(lightbikes
        src/main.cpp
        ${LIGHTBIKE_SRCS})

target_include_directories(lightbikes
        PUBLIC
        src
        ../yage/src
        ${SDL2_INCLUDE_DIRS})

target_link_libraries(lightbikes yage ${CONAN_LIBS})

set(ASSET_SOURCE "${PROJECT_SOURCE_DIR}/assets")
set(ASSET_DESTINATION "${PROJECT_BINARY_DIR}/assets")
add_custom_command(
        TARGET lightbikes POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E create_symlink ${ASSET_SOURCE} ${ASSET_DESTINATION}
        DEPENDS ${ASSET_DESTINATION}
        COMMENT "symbolic link resources folder from ${ASSET_SOURCE} => ${ASSET_DESTINATION}"
)

# TODO: Link lightbikes in as a library instead of listing all files

add_executable(lightbikes_test
        src/main_test.cpp
        ${LIGHTBIKE_SRCS}
        ${LIGHTBIKE_SRCS_TEST}
        )

target_include_directories(lightbikes_test
        PUBLIC
        src
        ../yage/src
        ${SDL2_INCLUDE_DIRS})

target_link_libraries(lightbikes_test yage ${CONAN_LIBS})