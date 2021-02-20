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
        src/main.cpp src/lightbikes/rendering/Renderable.hpp src/lightbikes/components/Position.hpp src/lightbikes/rendering/RenderSystem.cpp src/lightbikes/rendering/RenderSystem.hpp src/lightbikes/AppConfig.hpp src/lightbikes/states/LightbikeDuelState.cpp src/lightbikes/states/LightbikeDuelState.hpp src/lightbikes/input/InputSystem.cpp src/lightbikes/input/InputSystem.hpp src/lightbikes/input/InputState.hpp src/lightbikes/rendering/Camera.hpp src/lightbikes/components/Lightbike.hpp src/lightbikes/lightbikes/systems.hpp src/lightbikes/lightbikes/systems.cpp src/lightbikes/lightbikes/DuelConfig.hpp src/lightbikes/input/InputState.cpp src/lightbikes/components/LightbikePath.hpp)

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
