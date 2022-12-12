use std::collections::HashSet;

use aoc_common::FileContents;
use day8::Grid;
use day8::Direction;

fn setup(input_filepath: &str) -> Grid{
    let split_delim = "\n";

    let file_contents = 
        FileContents::build(input_filepath, split_delim, -1, -1)
            .unwrap_or_else(|err| {
                panic!("Unable to parse file: {err}");
            });
    Grid::build(&file_contents.split_contents)
}

fn main() {
    let grid = setup("input/day8/final_input.txt");

    let mut total_tree_count: u32 = 0;

    let mut hashset: HashSet<usize> = HashSet::new();

    // Left and right sides
    for i in 0..grid.height {
        total_tree_count += grid.count_trees((0, i), Direction::RIGHT, &mut hashset);
        total_tree_count += grid.count_trees((grid.width-1, i), Direction::LEFT, &mut hashset);
    }

    // Top and bottom sides
    for j in 0..grid.width {
        total_tree_count += grid.count_trees((j, 0), Direction::DOWN, &mut hashset);
        total_tree_count += grid.count_trees((j, grid.height-1), Direction::UP, &mut hashset);
    }

    for num in &hashset {
        println!("{}", num);
    }

    println!("Part 1: Total trees visible: {}", total_tree_count);

}
