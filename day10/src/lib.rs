
use std::fs;
use std::io::{self, Write};

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    fn test_setup() -> Vec<String> {
        let test_input_filepath = "../input/day10/final_input.txt";
        let contents = fs::read_to_string(test_input_filepath).unwrap();

        contents.split("\n").map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_cycles(){
        let mut lines = test_setup().into_iter();

        let mut X_val = 1;
        let mut cycle_no = 0;
        let mut current_cmd_cycles_left  = 0; //Flag to wait for n cycles
        let mut temp_val_to_add = 0;

        let mut signal_strength_sum = 0;

        loop {
            // write!(io::stdout(), "START [cycle_no, X_val]: [{},{}]\n", cycle_no, X_val).unwrap();

            if (cycle_no-20) % 40 == 0 {
                signal_strength_sum += cycle_no * X_val;
                write!(io::stdout(), "  TRACKING [cycle_no, X_val]: [{},{}]\n", cycle_no, X_val).unwrap();
            }

            if current_cmd_cycles_left > 0 {
                current_cmd_cycles_left -= 1;
                // write!(io::stdout(), "END [cycle_no, X_val]: [{},{}]\n", cycle_no, X_val).unwrap();
            }
            else {
                X_val += temp_val_to_add;
                temp_val_to_add = 0;
    
                // write!(io::stdout(), "END [cycle_no, X_val]: [{},{}]\n", cycle_no, X_val).unwrap();
    
                if let Some(line) = lines.next(){
                    let mut cmds = line.split(" ").into_iter();
                    match cmds.next() {
                        Some("noop") => {
                            // write!(io::stdout(), "Noop\n").unwrap();
                            current_cmd_cycles_left = 0;
                        },
                        Some("addx") => {
                            // write!(io::stdout(), "addx\n").unwrap();
                            temp_val_to_add = cmds.next().unwrap().parse::<i32>().unwrap();
        
                            current_cmd_cycles_left = 1;
                        },
                        _ => {
                            write!(io::stdout(), "Invalid command \n").unwrap(); 
                        }
                    };
                }
                else {
                    // End of input
                    write!(io::stdout(), "End of output \n").unwrap(); 
                    break;
                }
            }

            cycle_no += 1;
        }
        
        write!(io::stdout(), "Total signal strength: {}\n", signal_strength_sum).unwrap(); 

        ()
    }


}   



