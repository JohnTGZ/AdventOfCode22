#include <aoc_common.h>
#include <memory>

int main(int argc, char** argv){
    std::string filepath = "../input/day9/test_input.txt";

    auto logger = std::make_shared<Logger>(LOG_LEVEL::DEBUG);

    auto head_map = GridMap(6, 5);
    auto tail_map = GridMap(6, 5);

    head_map.display(logger);

    FileParser fileparser(filepath, logger);


    fileparser.split("\n");

}
