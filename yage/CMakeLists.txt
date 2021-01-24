cmake_minimum_required(VERSION 3.10)
# specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

SET_PROPERTY(GLOBAL PROPERTY TARGET_SUPPORTS_SHARED_LIBS TRUE)
find_package(SDL2 REQUIRED COMPONENTS main)

# set the project name
project(yage VERSION 0.0)

configure_file(YageConfig.h.in YageConfig.h)

# add the executable
add_executable(yage yage.cxx)
target_include_directories(yage PUBLIC
        "${PROJECT_BINARY_DIR}"
        ${SDL2_INCLUDE_DIRS}
        )

target_link_libraries(yage ${SDL2_LIBRARIES})