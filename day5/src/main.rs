use day5::Instructions;
use day5::Shipyard;
use day5::Moves;
use aoc_common::FileContents;

fn main() {
    let input_filepath = "input/day5/final_input.txt";
    let split_delim = "\n";

    let file_contents_shipyard = 
        FileContents::build(input_filepath, split_delim, 0, 8)
            .unwrap_or_else(|err| {
                panic!("Unable to parse file: {err}");
            });

    let file_contents_instructions = 
    FileContents::build(input_filepath, split_delim, 10, -1)
        .unwrap_or_else(|err| {
            panic!("Unable to parse file: {err}");
        });

    let (instructions, mut shipyard_9000) = 
        (Instructions::build(&file_contents_instructions), Shipyard::build(&file_contents_shipyard, 9));
    
    for single_move in &instructions.moves{
        shipyard_9000.move_crates(&single_move, false);
    }
    
    println!("Part 1: Shipyard top crate string using CrateMover 9000 [{}]", shipyard_9000.get_top_crate_string());

    let (_, mut shipyard_9001) = 
        (Instructions::build(&file_contents_instructions), Shipyard::build(&file_contents_shipyard, 9));

    for single_move in &instructions.moves{
        shipyard_9001.move_crates(&single_move, true);
    }

    println!("Part 2: Shipyard top crate string using CrateMover 9001 [{}]", shipyard_9001.get_top_crate_string());
    
}
