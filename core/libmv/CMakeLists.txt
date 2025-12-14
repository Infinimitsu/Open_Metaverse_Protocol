cmake_minimum_required(VERSION 3.20)
project(libmv_rust)

if(WIN32)
    # On Windows, Rust produces "libmv.dll" AND "libmv.dll.lib"
    set(LIB_FILE "libmv.dll")
    set(LIB_IMPORT "libmv.dll.lib")
elseif(APPLE)
    set(LIB_FILE "liblibmv.dylib")
else()
    set(LIB_FILE "liblibmv.so")
endif()

# 1. Define the Command to Build Rust
# We use 'cargo build' which puts artifacts in /target/debug
add_custom_target(cargo_build ALL
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    BYPRODUCTS "${CMAKE_CURRENT_SOURCE_DIR}/target/debug/${LIB_FILE}"
    COMMENT "Building Rust Core..."
)

# 2. Link Rust Code
add_library(libmv SHARED IMPORTED GLOBAL)
add_dependencies(libmv cargo_build)

# 3. Tell CMake EXACTLY where the files are
if(WIN32)
    set_target_properties(libmv PROPERTIES
        IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/target/debug/${LIB_FILE}"
        IMPORTED_IMPLIB "${CMAKE_CURRENT_SOURCE_DIR}/target/debug/${LIB_IMPORT}"
    )
else()
    set_target_properties(libmv PROPERTIES
        IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/target/debug/${LIB_FILE}"
    )
endif()

# 4. Expose Header Path
target_include_directories(libmv INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/include")