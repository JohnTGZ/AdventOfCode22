use aoc_common::FileContents;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_rucksack() {
        let test_input_filepath = "../input/day3/test_input.txt";

        let rucksacks = Rucksack::build(test_input_filepath);

        assert_eq!("vJrwpWtwJgWr", rucksacks[0].first);
        assert_eq!("hcsFMMfFFhFp", rucksacks[0].second);
        assert_eq!("jqHRNqRjqzjGDLGL", rucksacks[1].first);
        assert_eq!("vPwwTWBwg", rucksacks[2].second);
    }

    #[test]
    fn test_common_letter() {
        let test_input_filepath = "../input/day3/test_input.txt";

        let rucksacks = Rucksack::build(test_input_filepath);

        assert_eq!(Some('p'), rucksacks[0].get_common_letter());
        assert_eq!(Some('L'), rucksacks[1].get_common_letter());
        assert_eq!(Some('P'), rucksacks[2].get_common_letter());
        assert_eq!(Some('v'), rucksacks[3].get_common_letter());
        assert_eq!(Some('t'), rucksacks[4].get_common_letter());
        assert_eq!(Some('s'), rucksacks[5].get_common_letter());
    }


    #[test]
    fn test_priority() {

        assert_eq!(1, Rucksack::get_priority_score(&'a'));
        assert_eq!(26, Rucksack::get_priority_score(&'z'));
        assert_eq!(16, Rucksack::get_priority_score(&'p'));
        assert_eq!(38, Rucksack::get_priority_score(&'L'));
        assert_eq!(42, Rucksack::get_priority_score(&'P'));
        assert_eq!(22, Rucksack::get_priority_score(&'v'));
        assert_eq!(20, Rucksack::get_priority_score(&'t'));
        assert_eq!(19, Rucksack::get_priority_score(&'s'));
        assert_eq!(27, Rucksack::get_priority_score(&'A'));
        assert_eq!(52, Rucksack::get_priority_score(&'Z'));
    }

    #[test]
    fn test_total_priority() {
        let test_input_filepath = "../input/day3/test_input.txt";

        let rucksacks = Rucksack::build(test_input_filepath);

        assert_eq!(157, Rucksack::get_total_priority(rucksacks));
    }
    
}

pub struct Rucksack{
    first: String,
    second: String,
}

impl Rucksack{
    pub fn build(input_filepath: &str) -> Vec<Rucksack>{
        let mut rucksacks: Vec<Rucksack> = Vec::new();

        let file_contents = 
            FileContents::build(input_filepath, "\n")
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });
        
        for line in file_contents.split_contents{
            let halfway_point = line.len()/2;
            let compartments = line.split_at(halfway_point);
            rucksacks.push(Rucksack{
                first: compartments.0.to_string(),
                second: compartments.1.to_string(),
            })
        }

        rucksacks
    }

    pub fn get_priority_score(letter: &char) -> u32{
        let unicode_val = *letter as u32;

        let lower_case = 65..91;
        let upper_case = 97..123;
        if lower_case.contains(&unicode_val) {
            return unicode_val - 38;
        }
        else if upper_case.contains(&unicode_val){
            return unicode_val - 96;
        }
        else {
            return 0
        }
    }

    pub fn get_common_letter(&self) -> Option<char> {
        // Method 1: Use a hashmap to increment the count

        // Store first compartment into a hashmap first
        let mut first_comp_hashmap: HashMap<char, i32> = HashMap::new();

        for letter in self.first.chars() {
            first_comp_hashmap.insert(letter, 1);
        }

        for letter in self.second.chars() {
            match first_comp_hashmap.get(&letter) {
                Some(&_) => {
                    return Some(letter);
                }, 
                _ => (),
            }
        }

        None
    }

    pub fn get_total_priority(rucksacks: Vec<Rucksack>) -> u64 {
        let mut total_priority: u64 = 0;
        for rucksack in rucksacks{
            total_priority += u64::from(
                Rucksack::get_priority_score(
                    &rucksack.get_common_letter().expect("No common letter")))
        }

        total_priority
    } 

}

