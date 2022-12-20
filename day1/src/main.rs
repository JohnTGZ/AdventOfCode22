use std::process;

use aoc_common::FileContents;
use day1::Elves;

fn main() {
    println!("Day 1");

    let final_input_filepath = "input/day1/final_input.txt";
    let split_delim = "\n\n";

    let file_contents = FileContents::build(final_input_filepath, split_delim, -1, -1)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing file: {err}");
            process::exit(1);
        });

    let elves = Elves::build(&file_contents);

    println!(
        "Part 1: Highest amount of calories [{}]",
        elves.get_max_calories()
    );

    println!(
        "Part 2: Top three calories summed [{}]",
        elves.get_top_three_calories_summed()
    );
}
