use std::fs;
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    fn test_setup() -> Vec<String> {
        let test_input_filepath = "../input/day10/test_input2.txt";
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
                // write!(io::stdout(), "  TRACKING [cycle_no, X_val]: [{},{}]\n", cycle_no, X_val).unwrap();
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
                    // write!(io::stdout(), "End of output \n").unwrap();
                    break;
                }
            }

            cycle_no += 1;
        }
        assert_eq!(signal_strength_sum, 13140);

        ()
    }

    #[test]
    fn test_sprite_display() {
        let mut lines = test_setup().into_iter();

        let mut X_val = 1; // This is the sprite position
        let mut cycle_no = 0;
        let mut current_cmd_cycles_left = 0; //Flag to wait for n cycles
        let mut temp_val_to_add = 0;

        let mut row = 0;
        let mut crt_display: Vec<String> = Vec::new();
        let mut crt_display_row: String = String::new();

        loop {
            if cycle_no > 0 {
                let cycle_pos = cycle_no - row * 40 - 1 ;

                // If position of sprite is within the cycle number
                if ((X_val-1)..=(X_val + 1)).contains(&cycle_pos) {
                    crt_display_row += "#";
                } 
                else {
                    crt_display_row += ".";
                }

                // Display string every 40 cycles and then clear it.
                if (cycle_no) % 40 == 0 {
                    write!(io::stdout(), "{crt_display_row}\n").unwrap();
                    crt_display.push(crt_display_row.clone());
                    crt_display_row.clear();
                    row += 1;
                }
            }

            // Still executing previous instruction
            if current_cmd_cycles_left > 0 {
                current_cmd_cycles_left -= 1;
            }
            // Read instructions
            else {
                X_val += temp_val_to_add;
                temp_val_to_add = 0;

                if let Some(line) = lines.next() {
                    let mut cmds = line.split(" ").into_iter();
                    match cmds.next() {
                        Some("noop") => {
                            current_cmd_cycles_left = 0;
                        }
                        Some("addx") => {
                            temp_val_to_add = cmds.next().unwrap().parse::<i32>().unwrap();
                            current_cmd_cycles_left = 1;
                        }
                        _ => {
                            write!(io::stdout(), "Invalid command \n").unwrap();
                        }
                    };
                } else {
                    // End of input
                    write!(io::stdout(), "End of output \n").unwrap();
                    break;
                }
            }

            // Increment cycle
            cycle_no += 1;
        }

        assert_eq!(
            crt_display[0],
            String::from("##..##..##..##..##..##..##..##..##..##..")
        );
        assert_eq!(
            crt_display[1],
            String::from("###...###...###...###...###...###...###.")
        );
        assert_eq!(
            crt_display[2],
            String::from("####....####....####....####....####....")
        );
        assert_eq!(
            crt_display[3],
            String::from("#####.....#####.....#####.....#####.....")
        );
        assert_eq!(
            crt_display[4],
            String::from("######......######......######......####")
        );
        assert_eq!(
            crt_display[5],
            String::from("#######.......#######.......#######.....")
        );

        ()
    }



}
