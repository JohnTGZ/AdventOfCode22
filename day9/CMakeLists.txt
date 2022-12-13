cmake_minimum_required(VERSION 3.10)

set (CMAKE_CXX_STANDARD 17)

project(raytracing_weekend)

include_directories(include)

add_library(aoc_common
    src/aoc_common.cpp
)
# https://stackoverflow.com/questions/26243169/cmake-target-include-directories-meaning-of-scope
target_include_directories(aoc_common
    PRIVATE include
)

add_executable(day9
    src/day9.cpp
)
target_include_directories(day9
    PRIVATE include
)
target_link_libraries(day9
    aoc_common
)
