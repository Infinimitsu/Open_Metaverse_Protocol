cmake_minimum_required(VERSION 3.20)
project(Open_Metaverse_Protocol)

# --- PROJECT STRUCTURE ---

# 1. Build the Core Library (LibMV)
# This processes core/libmv/CMakeLists.txt first
add_subdirectory(core/libmv)

# 2. Build the Visualizer Client
# This processes client/barebones/CMakeLists.txt next
add_subdirectory(client/barebones)