use aoc_common::FileContents;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_score_part1() {
        let test_input_filepath = "../input/day2/test_input.txt";
        let split_delim = "\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim)
                .unwrap_or_else(|err| {
                    eprintln!("Problem parsing file: {err}");
                    panic!("Unable to parse file");
                });

        let rps = RPS::build(&file_contents).expect("Error building a RPS (Rock,Papers,Scissors) object");

        assert_eq!(15, rps.get_total_score_part1())
    }

    #[test]
    fn test_get_total_score_part2() {
        let test_input_filepath = "../input/day2/test_input.txt";
        let split_delim = "\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim)
                .unwrap_or_else(|err| {
                    eprintln!("Problem parsing file: {err}");
                    panic!("Unable to parse file");
                });

        let rps = RPS::build(&file_contents).expect("Error building a RPS (Rock,Papers,Scissors) object");

        assert_eq!(12, rps.get_total_score_part2())
    }
}

struct ShapeMetadata {
    shape_score: u8,
    draw: &'static str,
    win: &'static str,
}

pub struct RPS {
    // TODO: Improvement by changing String to char
    shape_metadata_hash: HashMap<String, ShapeMetadata>,
    rps_moves: Vec<[String;2]> // Stores the moves made 
}

impl RPS {
    pub fn build(file_contents: &FileContents) -> Option<RPS> {
        let mut rps_moves: Vec<[String;2]> = Vec::new();

        for chunk in &file_contents.split_contents {
            let mut line = chunk.split(" ");

            let lhs: String = line.next().unwrap().to_string();
            let rhs: String = line.next().unwrap().to_string();

            rps_moves.push([lhs, rhs]);
        }

        let mut shape_metadata_hash: HashMap<String, ShapeMetadata> = HashMap::new();
        shape_metadata_hash.insert(
            "X".to_string(), 
            ShapeMetadata{
                shape_score: 1,
                draw: "A",
                win: "C",
            }
        );
        shape_metadata_hash.insert(
            "Y".to_string(), 
            ShapeMetadata{
                shape_score: 2,
                draw: "B",
                win: "A",
            }
        );
        shape_metadata_hash.insert(
            "Z".to_string(), 
            ShapeMetadata{
                shape_score: 3,
                draw: "C",
                win: "B",
            }
        );

        println!("Constructed RPS object");
        for (shape_name, meta_data) in shape_metadata_hash.iter() {
            println!("{} : Score[{}], Wins[{}], Draws[{}] ", 
                shape_name, 
                meta_data.shape_score,
                meta_data.win,
                meta_data.draw);
        }

        Some(
            RPS{
                shape_metadata_hash: shape_metadata_hash,
                rps_moves: rps_moves,
            }
        )
    }

    pub fn get_score(&self, shape_elf: &str, shape_self: &str) -> Option<u8>{
        let mut score: u8 = 0;

        // Add score for shape choosed
        match self.shape_metadata_hash.get(shape_self){
            Some(shape_metadata) => {
                score += shape_metadata.shape_score;

                if shape_metadata.win == shape_elf {
                    score += 6;
                } 
                else if shape_metadata.draw == shape_elf {
                    score += 3;
                }
            },
            _ => {
                println!("Invalid shape!");
                return None;
            },
        }

        Some(score)
    }
    
    pub fn get_score_part2(&self, shape_elf: &str, shape_self: &str) -> Option<u8>{
        let mut score: u8 = 0;

        // Add score for shape choosed
        match self.shape_metadata_hash.get(shape_self){
            Some(shape_metadata) => {
                score += shape_metadata.shape_score;

                if shape_metadata.win == shape_elf {
                    score += 6;
                } 
                else if shape_metadata.draw == shape_elf {
                    score += 3;
                }
            },
            _ => {
                println!("Invalid shape!");
                return None;
            },
        }

        Some(score)
    }

    pub fn get_total_score_part1(&self) -> u64 {
        let mut total_score: u64 = 0;
        for rps_move in &self.rps_moves {
            let match_score = self.get_score(&rps_move[0], &rps_move[1])
                .unwrap_or_else(||{
                    panic!("Unable to get score!");
                });
            total_score += u64::from(match_score);
        }
        total_score
    }

    pub fn get_total_score_part1(&self) -> u64 {
        let mut total_score: u64 = 0;
        for rps_move in &self.rps_moves {
            let match_score = self.get_score_part2(&rps_move[0], &rps_move[1])
                .unwrap_or_else(||{
                    panic!("Unable to get score!");
                });
            total_score += u64::from(match_score);
        }
        total_score
    }


}



