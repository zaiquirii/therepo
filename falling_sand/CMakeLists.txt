cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
SET(CONAN_DISABLE_CHECK_COMPILER true)
include(./build/conanbuildinfo.cmake)
conan_basic_setup()
SET_PROPERTY(GLOBAL PROPERTY TARGET_SUPPORTS_SHARED_LIBS TRUE)
find_package(SDL2 REQUIRED COMPONENTS main)

project(src/falling_sand VERSION 0.1)

add_subdirectory(../yage ${PROJECT_BINARY_DIR}/build_yage)

add_executable(falling_sand
        src/falling_sand/main.cpp
        src/falling_sand/sim/cell.hpp
        src/falling_sand/sim/cell.cpp
        src/falling_sand/sim/SandboxConfig.hpp
        src/falling_sand/sim/CellSim.cpp
        src/falling_sand/sim/CellSim.hpp
        src/falling_sand/ui/Brush.cpp
        src/falling_sand/ui/Brush.hpp
        src/falling_sand/ui/InputSystem.cpp
        src/falling_sand/ui/InputSystem.hpp
        src/falling_sand/sim/cells/cells.hpp
        src/falling_sand/sim/cells/cells.cpp
        src/falling_sand/ui/Toolbox.cpp
        src/falling_sand/ui/Toolbox.hpp src/falling_sand/MainState.cpp src/falling_sand/MainState.hpp src/falling_sand/CellSystem.cpp src/falling_sand/CellSystem.hpp)

target_include_directories(falling_sand
        PUBLIC
        src
        ../yage/src
        ${SDL2_INCLUDE_DIRS})

target_link_libraries(falling_sand yage ${CONAN_LIBS})

set(ASSET_SOURCE "${PROJECT_SOURCE_DIR}/assets")
set(ASSET_DESTINATION "${PROJECT_BINARY_DIR}/assets")
add_custom_command(
        TARGET falling_sand POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E create_symlink ${ASSET_SOURCE} ${ASSET_DESTINATION}
        DEPENDS ${ASSET_DESTINATION}
        COMMENT "symbolic link resources folder from ${ASSET_SOURCE} => ${ASSET_DESTINATION}"
)
