use aoc_common::FileContents;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_calories() {
        let test_input_filepath = "../input/day1/test_input.txt";
        let split_delim = "\n\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim)
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });

        let elves = Elves::build(&file_contents);

        assert_eq!(24000, elves.get_max_calories())
    }

    #[test]
    fn test_top_three_calories_summed() {
        let test_input_filepath = "../input/day1/test_input.txt";
        let split_delim = "\n\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim)
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });

        let elves = Elves::build(&file_contents);

        assert_eq!(45000, elves.get_top_three_calories_summed())
    }
}

pub struct Elves {
    pub calories: Vec<i32>,
}

impl Elves {
    /// Parse total calories for each elf
    pub fn build(file_contents: &FileContents) -> Elves{
        let mut calories: Vec<i32> = Vec::new();

        for chunk in &file_contents.split_contents {
            let summed_calories: i32 = chunk
                                            .lines()
                                            .map(|line| line.parse::<i32>().unwrap())
                                            .sum();
            calories.push(summed_calories);
        }

        Elves{
            calories: calories,
        }
    }

    /// Get the the top calorie count of a single elf
    pub fn get_max_calories(&self) -> i32{
        if let Some(&max) = self.calories.iter().max() {
            max
        } 
        else {
            -1
        }
    }

    /// Get the sum of the top three calorie counts 
    pub fn get_top_three_calories_summed(&self) -> i32{
        let mut calories_sorted = self.calories.clone();
        calories_sorted.sort_unstable(); 

        &calories_sorted[calories_sorted.len()-1] 
            + &calories_sorted[calories_sorted.len()-2] 
            + &calories_sorted[calories_sorted.len()-3]
    }

}


