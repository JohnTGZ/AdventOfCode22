use aoc_common::FileContents;
use day4::Assignments;

fn main() {
    println!("Day 4");

    let input_filepath = "input/day4/final_input.txt";
    let split_delim = "\n";

    let file_contents =
        FileContents::build(input_filepath, split_delim, -1, -1).unwrap_or_else(|err| {
            panic!("Unable to parse file: {err}");
        });

    let assignments = Assignments::build(&file_contents);

    println!(
        "Part 1: Total fully-contained pairs [{}]",
        assignments.count_fully_contains()
    );

    println!(
        "Part 2: Total partially-contained pairs [{}]",
        assignments.count_partially_contains()
    );
}
