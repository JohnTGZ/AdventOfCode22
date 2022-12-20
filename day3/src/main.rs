use day3::Rucksack;

fn main() {
    println!("Day 3");

    let input_filepath = "input/day3/final_input.txt";

    let rucksacks = Rucksack::build(input_filepath);

    println!(
        "Part 1: Total priority [{}]",
        Rucksack::get_total_priority(&rucksacks)
    );

    println!(
        "Part 2: Total priority [{}]",
        Rucksack::get_total_priority_part2(&rucksacks)
    );
}
