#include <aoc_common.h>
#include <memory>

int main(int argc, char** argv){

    auto logger = std::make_shared<Logger>(LOG_LEVEL::DEBUG);
    auto fileparser = FileParser("../input/day9/final_input.txt", logger);
    const auto lines = fileparser.split('\n');

    Position2D min_pos(0, 0);
    Position2D max_pos(0, 0);
    Position2D position_now(0, 0);

    // Get map size
    for (int i = 0; i < lines.size(); i++){
        const auto inst = FileParser::split(lines[i], ' ');
        const auto movement = Vector2D(inst[0], std::stoi(inst[1].c_str()));
        
        position_now += movement.to_position();

        max_pos.x = (position_now.x > max_pos.x) ? position_now.x : max_pos.x;
        max_pos.y = (position_now.y > max_pos.y) ? position_now.y : max_pos.y;

        min_pos.x = (position_now.x < min_pos.x) ? position_now.x : min_pos.x;
        min_pos.y = (position_now.y < min_pos.y) ? position_now.y : min_pos.y;
    }
    long width = max_pos.x - min_pos.x + 1;
    long height = max_pos.y - min_pos.y + 1;

    auto gridmap = GridMap(width,height);
    
    gridmap.init(Position2D(-min_pos.x, -min_pos.y), 1);

    for (int i = 0; i < lines.size(); i++){
        const auto inst = FileParser::split(lines[i], ' ');
        std::cout << lines[i] << std::endl;
        gridmap.move_all_cells(
            inst[0], std::stoi(inst[1].c_str()));
    }

    std::cout << "Part 1: No. of cells visited by tail: " << gridmap.get_num_visited() << std::endl;
    
}
