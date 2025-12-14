cmake_minimum_required(VERSION 3.20)
project(mv_visualizer)

# --- DEPENDENCIES ---

# 1. LibMV (Our Core)
# The 'libmv' target is inherited from the root build.

# 2. Raylib
include(FetchContent)
FetchContent_Declare(
    raylib
    URL https://github.com/raysan5/raylib/archive/master.tar.gz
)
FetchContent_MakeAvailable(raylib)

# --- BUILD EXECUTABLE ---

add_executable(mv_client main.cpp)

# Link Libraries
target_link_libraries(mv_client PRIVATE 
    raylib 
    libmv
)

# Copy the libmv DLL to the executable folder post-build
if(WIN32)
    add_custom_command(TARGET mv_client POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:libmv>
        $<TARGET_FILE_DIR:mv_client>
    )
endif()