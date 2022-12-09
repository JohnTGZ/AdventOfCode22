use aoc_common::FileContents;
use std::collections::HashMap;


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> RPS {
        let test_input_filepath = "../input/day2/test_input.txt";
        let split_delim = "\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim, -1, -1)
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });

        RPS::build(&file_contents)
            .expect("Error building a RPS (Rock,Papers,Scissors) object")
    }

    #[test]
    fn test_get_total_score_part1() {
        let rps = setup();

        assert_eq!(15, rps.get_total_score_part1())
    }

    #[test]
    fn test_get_total_score_part2() {
        let rps = setup();

        assert_eq!(12, rps.get_total_score_part2())
    }
}

struct ShapeMetadata {
    shape_score: u8,
    draw: &'static str,
    win: &'static str,
    loses_to: &'static str,
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
                loses_to: "B",
            }
        );
        shape_metadata_hash.insert(
            "Y".to_string(), 
            ShapeMetadata{
                shape_score: 2,
                draw: "B",
                win: "A",
                loses_to: "C",
            }
        );
        shape_metadata_hash.insert(
            "Z".to_string(), 
            ShapeMetadata{
                shape_score: 3,
                draw: "C",
                win: "B",
                loses_to: "A",
            }
        );

        // For part 2, 
        shape_metadata_hash.insert(
            "A".to_string(), 
            ShapeMetadata{
                shape_score: 1,
                draw: "X",
                win: "Z",
                loses_to: "Y",
            }
        );
        shape_metadata_hash.insert(
            "B".to_string(), 
            ShapeMetadata{
                shape_score: 2,
                draw: "Y",
                win: "X",
                loses_to: "Z",
            }
        );
        shape_metadata_hash.insert(
            "C".to_string(), 
            ShapeMetadata{
                shape_score: 3,
                draw: "Z",
                win: "Y",
                loses_to: "X",
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

    pub fn get_score_part1(&self, shape_elf: &str, shape_self: &str) -> Option<u8>{
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
    
    pub fn get_score_part2(&self, shape_elf: &str, match_outcome: &str) -> Option<u8>{
        let mut score: u8 = 0;

        let elf_shape_metadata = 
            self.shape_metadata_hash
                .get(shape_elf)
                .expect("Shape not found!");

        match match_outcome {
            "X" => { // Lose
                score += self.shape_metadata_hash
                            .get(elf_shape_metadata.win)
                            .expect("Shape not found!")
                            .shape_score;
            },
            "Y" => { // End in draw
                score += 3 + self.shape_metadata_hash
                                .get(elf_shape_metadata.draw)
                                .expect("Shape not found!")
                                .shape_score;
            },
            "Z" => { // Win
                score += 6 + self.shape_metadata_hash
                                .get(elf_shape_metadata.loses_to)
                                .expect("Shape not found!")
                                .shape_score;
            }
            _ => panic!("Invalid match outcome"),
        }

        Some(score)
    }

    pub fn get_total_score_part1(&self) -> u64 {
        let mut total_score: u64 = 0;
        for rps_move in &self.rps_moves {
            let match_score = self.get_score_part1(&rps_move[0], &rps_move[1])
                .unwrap_or_else(||{
                    panic!("Unable to get score!");
                });
            total_score += u64::from(match_score);
        }
        total_score
    }

    pub fn get_total_score_part2(&self) -> u64 {
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



