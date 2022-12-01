use std::error::Error;
use std::fs;
use std::process;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_calories() {
        let test_input1 = "test_input1.txt";

        let elves = Elves::build(test_input1).unwrap_or_else(|err| {
            eprintln!("Problem parsing file: {err}");
            process::exit(1);
        });

        assert_eq!(24000, get_max_calories(&elves))
    }

    #[test]
    fn test_top_three_calories_summed() {
        let test_input1 = "test_input1.txt";

        let elves = Elves::build(test_input1).unwrap_or_else(|err| {
            eprintln!("Problem parsing file: {err}");
            process::exit(1);
        });

        assert_eq!(45000, get_top_three_calories_summed(&elves))
    }
}

pub struct Elves {
    pub input_filepath: String,
    pub calories: Vec<i32>,
}

impl Elves {
    /// Parse the file to get total calories for each elf
    pub fn build(input_filepath: &str) -> Result <Elves, Box<dyn Error>>
    {
        let contents = fs::read_to_string(input_filepath)?;

        let elves_calories = contents.split("\n\n");

        let mut calories: Vec<i32> = Vec::new();

        elves_calories
            .for_each(|elf|
                {
                    let summed_calories: i32 = elf
                        .lines()
                        .map(|line| line.parse::<i32>().unwrap())
                        .sum();
                    // println!("====");
                    // println!("{summed_calories}");
                    calories.push(summed_calories);
                }
            );

        Ok(Elves{
            input_filepath: input_filepath.to_string(),
            calories: calories,
        })
    }
}

pub fn get_max_calories(elves: &Elves) -> i32{
    if let Some(&max) = elves.calories.iter().max(){
        max
    } 
    else {
        -1
    }
}

pub fn get_top_three_calories_summed(elves: &Elves) -> i32{
    let mut calories = elves.calories.clone();
    calories.sort_unstable(); 

    &calories[calories.len()-1] 
        + &calories[calories.len()-2] 
        + &calories[calories.len()-3]
}

fn main() {
    let final_input = "input.txt";

    let elves = Elves::build(final_input).unwrap_or_else(|err| {
        eprintln!("Problem parsing file: {err}");
        process::exit(1);
    });

    let highest_calories = get_max_calories(&elves);

    println!("Part 1: Highest amount of calories [{highest_calories}]");

    let top_three_calories_summed = get_top_three_calories_summed(&elves);

    println!("Part 2: Top three calories summed [{top_three_calories_summed}]");

}
