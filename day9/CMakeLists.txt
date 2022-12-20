cmake_minimum_required(VERSION 3.14)
project(aoc22)

set (CMAKE_CXX_STANDARD 17)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
FetchContent_MakeAvailable(googletest)

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

add_executable(day9_p2
    src/day9_p2.cpp
)
target_include_directories(day9_p2
    PRIVATE include
)
target_link_libraries(day9_p2
    aoc_common
)

enable_testing()
add_executable(
    test_day9
    tests/test_day9.cpp
)
target_link_libraries(
    test_day9
    GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(test_day9)
