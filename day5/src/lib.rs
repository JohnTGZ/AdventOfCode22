use aoc_common::FileContents;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Instructions, Shipyard) {
        let test_input_filepath = "../input/day5/test_input.txt";
        let split_delim = "\n";

        let file_contents_instructions = 
            FileContents::build(test_input_filepath, split_delim, 5, -1)
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });

        let file_contents_shipyard = 
            FileContents::build(test_input_filepath, split_delim, 0, 3)
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });

        // file_contents.print();
        (Instructions::build(&file_contents_instructions), Shipyard::build(&file_contents_shipyard, 3))
        
    }

    #[test]
    fn test_read_moves() {
        let (instructions, _) = setup();
        
        assert_eq!(Moves {from: 1, to: 0, quantity: 1,}, instructions.moves[0]);
        assert_eq!(Moves {from: 0, to: 2, quantity: 3,}, instructions.moves[1]);
        assert_eq!(Moves {from: 0, to: 1, quantity: 1,}, instructions.moves[3]);
    }

    #[test]
    fn test_read_stacks() {
        let (_, shipyard) = setup();
        
        assert_eq!('Z', shipyard.stacks[0][0]);
        assert_eq!('N', shipyard.stacks[0][1]);

        assert_eq!('M', shipyard.stacks[1][0]);
        assert_eq!('C', shipyard.stacks[1][1]);
        assert_eq!('D', shipyard.stacks[1][2]);

        assert_eq!('P', shipyard.stacks[2][0]);
    }

    #[test]
    fn test_move_crates() {
        let (instructions, mut shipyard) = setup();
        
        for single_move in instructions.moves{
            shipyard.move_crates(single_move, false);
        }
        
        assert_eq!('C', *shipyard.stacks[0].last().unwrap());
        assert_eq!('M', *shipyard.stacks[1].last().unwrap());
        assert_eq!('Z', *shipyard.stacks[2].last().unwrap());
    }

    #[test]
    fn extract_message() {
        let (instructions, mut shipyard) = setup();
        
        for single_move in instructions.moves{
            shipyard.move_crates(single_move, false);
        }
        
        assert_eq!("CMZ", shipyard.get_top_crate_string());
    }

    #[test]
    fn test_move_crates_part2() {
        let (instructions, mut shipyard) = setup();
        
        for single_move in instructions.moves{
            shipyard.move_crates(single_move, true);
        }
        
        assert_eq!('C', *shipyard.stacks[0].last().unwrap());
        assert_eq!('M', *shipyard.stacks[1].last().unwrap());
        assert_eq!('Z', *shipyard.stacks[2].last().unwrap());
    }



}

#[derive(Debug)]
pub struct Shipyard{
    //Change to pointer of stacks?
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
pub struct Instructions{
    pub moves: Vec<Moves>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Moves{
    from : usize,
    to : usize,
    quantity: usize,
}

impl Shipyard {
    pub fn build(file_contents: &FileContents, num_stacks: usize) -> Shipyard{

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);

        //Populate with empty vectors
        for _ in 0..num_stacks{
            let stack: Vec<char> = Vec::new();
            stacks.push(stack);
        }

        for line in file_contents.split_contents.clone().into_iter().rev(){
            for idx in (1..line.len()-1).step_by(4){
                // Assumes input is ASCII
                let crate_id = line.as_bytes()[idx];
                if crate_id == b' '{
                    continue
                }
                let stack_idx = (idx-1)/4; 
                // println!("Pushed {} to Stack {}", 
                //     crate_id as char, stack_idx);
                stacks[stack_idx].push(crate_id as char)
            }
        }

        Shipyard{
            stacks,
        }
    }

    pub fn move_crates(&mut self, mv: &Moves, move_multiple: bool) -> () {
        println!("Moving {} crates from {} to {}",
            mv.quantity, mv.from, mv.to);
        let from_stack_final_size = self.stacks[mv.from].len().saturating_sub(mv.quantity);

        // let stacks = &self.stacks;
        let mut moved_crates: Vec<char> = self.stacks[mv.from].split_off(from_stack_final_size);
        if !move_multiple{
            moved_crates.reverse();
        }

        self.stacks[mv.to].append(&mut moved_crates);
    }

    pub fn get_top_crate_string(self) -> String {
        let mut top_crate_string = String::new();
        for stack in self.stacks{
            top_crate_string += &(stack.last().unwrap().to_string());
        }
        top_crate_string
    }
}

impl Instructions {
    pub fn build(file_contents: &FileContents) -> Instructions{
        let mut moves: Vec<Moves> = Vec::new();

        let re = Regex::new(
            r"(\w+) ([0-9]+) (\w+) ([0-9]+) (\w+) ([0-9]+)",
        ).expect("Unable to form regex");
        
        for cap in re.captures_iter(&file_contents.contents)
        {
            moves.push(
                Moves { 
                    from: (&cap[4]).parse::<usize>().unwrap() - 1, 
                    to: (&cap[6]).parse::<usize>().unwrap() - 1, 
                    quantity: (&cap[2]).parse::<usize>().unwrap(), 
                }
            )
        }

        Instructions{
            moves,
        }
    }
}


