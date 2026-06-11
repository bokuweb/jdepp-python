cmake_minimum_required(VERSION 3.16)

set(EXE_TARGET "jdepp")
project(${EXE_TARGET} CXX)

# cmake modules
list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)
list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake/sanitizers)
find_package(Sanitizers) # Address sanitizer (-DSANITIZE_ADDRESS=ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(TARGET jdepp)

add_library(${TARGET}
  STATIC
    crates/jdepp-rs/src/bindings.cpp
    jdepp/classify.cc
    jdepp/classify.h
    jdepp/kernel.cc
    jdepp/kernel.h
    jdepp/linear.cc
    jdepp/linear.h
    jdepp/pdep.cc
    jdepp/pdep.h
    jdepp/pecco.h
    jdepp/pa.h
    jdepp/cedar.h
    jdepp/io-util.cc
    jdepp/io-util.hh
)

install (TARGETS ${TARGET} DESTINATION .)