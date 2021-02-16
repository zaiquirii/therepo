cmake_minimum_required(VERSION 3.10)
# specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
# TODO: Find out if there is a better way to fix this issue
SET(CONAN_DISABLE_CHECK_COMPILER true)
include(./build/conanbuildinfo.cmake)
conan_basic_setup()

SET_PROPERTY(GLOBAL PROPERTY TARGET_SUPPORTS_SHARED_LIBS TRUE)
find_package(SDL2 REQUIRED COMPONENTS main)

# set the project name
project(yage VERSION 0.1)


configure_file(YageConfig.h.in YageConfig.h)
set(YAGE_HEADERS ./include/yage.h)

add_library(yage
        STATIC
        src/yage/window/WindowConfig.h
        src/yage/window/window.cpp src/yage/Game.hpp src/yage/GameState.hpp src/yage/Game.cpp src/yage/GameSystem.hpp src/yage/World.cpp src/yage/World.hpp src/yage/Resources.cpp src/yage/Resources.hpp src/yage/Family.hpp src/yage/NoCopy.hpp src/yage/YageException.cpp src/yage/YageException.hpp src/yage/Time.cpp src/yage/Time.hpp src/yage/FrameRateLimiter.cpp src/yage/FrameRateLimiter.hpp)
set_target_properties(yage PROPERTIES LINKER_LANGUAGE CXX)

# add the executable
target_include_directories(yage PUBLIC
        "./include"
        ${PROJECT_BINARY_DIR}
        ${SDL2_INCLUDE_DIRS}
        )

set(YAGE_INCLUDE_DIRS
        "./include")

target_link_libraries(yage ${SDL2_LIBRARIES} ${CONAN_LIBS})

#set(ASSET_SOURCE "${PROJECT_SOURCE_DIR}/assets")
#set(ASSET_DESTINATION "${PROJECT_BINARY_DIR}/assets")
#add_custom_command(
#        TARGET yage POST_BUILD
#        COMMAND ${CMAKE_COMMAND} -E create_symlink ${ASSET_SOURCE} ${ASSET_DESTINATION}
#        DEPENDS ${ASSET_DESTINATION}
#        COMMENT "symbolic link resources folder from ${ASSET_SOURCE} => ${ASSET_DESTINATION}"
#)