use std::fs;
use std::io::{self, Write};

fn main() {
    let input_filepath = "input/day10/final_input.txt";

    let contents: Vec<String> = fs::read_to_string(input_filepath)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut lines = contents.clone().into_iter();

    let mut X_val = 1;
    let mut cycle_no = 0;
    let mut current_cmd_cycles_left = 0; //Flag to wait for n cycles
    let mut temp_val_to_add = 0;

    let mut signal_strength_sum = 0;

    loop {
        if (cycle_no - 20) % 40 == 0 {
            signal_strength_sum += cycle_no * X_val;
        }

        if current_cmd_cycles_left > 0 {
            current_cmd_cycles_left -= 1;
        } else {
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

        cycle_no += 1;
    }
    println!("Part 1: {}", signal_strength_sum);

    println!("Part 2: ");

    let mut lines = contents.into_iter();

    let mut X_val = 1; // This is the sprite position
    let mut cycle_no = 0;
    let mut current_cmd_cycles_left = 0; //Flag to wait for n cycles
    let mut temp_val_to_add = 0;

    let mut row = 0;
    let mut crt_display: Vec<String> = Vec::new();
    let mut crt_display_row: String = String::new();

    loop {
        if cycle_no > 0 {
            let cycle_pos = cycle_no - row * 40 - 1;

            // If position of sprite is within the cycle number
            if ((X_val - 1)..=(X_val + 1)).contains(&cycle_pos) {
                crt_display_row += "#";
            } else {
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
}
