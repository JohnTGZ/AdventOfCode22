use aoc_common::FileContents;
use day2::RPS;

fn main() {
    println!("Day 2");

    let test_input_filepath = "input/day2/final_input.txt";
    let split_delim = "\n";

    let file_contents = 
        FileContents::build(test_input_filepath, split_delim)
            .unwrap_or_else(|err| {
                eprintln!("Problem parsing file: {err}");
                panic!("Unable to parse file");
            });

    let rps = RPS::build(&file_contents).expect("Error building a RPS (Rock,Papers,Scissors) object");

    println!("Part 1: Score from following strategy [{}]", rps.get_total_score_part1());


    // println!("Part 2: [{}]", rps.get_total_score());

}
