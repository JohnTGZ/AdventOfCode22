use std::error::Error;
use std::fs;
use std::process;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calories() {
        let test_input1 = "test_input1.txt";

        let elves = Elves::build(test_input1).unwrap_or_else(|err| {
            eprintln!("Problem parsing file: {err}");
            process::exit(1);
        });

        assert_eq!(24000, get_max_calories(&elves))
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

fn main() {
    let final_input = "input.txt";

    let elves = Elves::build(final_input).unwrap_or_else(|err| {
        eprintln!("Problem parsing file: {err}");
        process::exit(1);
    });

    let highest_calories = get_max_calories(&elves);

    println!("Highest amount of calories: {highest_calories}")
}
