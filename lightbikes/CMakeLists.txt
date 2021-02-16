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
add_executable(lightbikes
        src/main.cpp)

target_include_directories(lightbikes
        PUBLIC
        src
        ../yage/src
        ${SDL2_INCLUDE_DIRS})

target_link_libraries(lightbikes yage ${CONAN_LIBS})
