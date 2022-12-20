use aoc_common::FileContents;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_setup() -> Grid {
        let test_input_filepath = "../input/day8/test_input.txt";
        let split_delim = "\n";

        let file_contents = FileContents::build(test_input_filepath, split_delim, -1, -1)
            .unwrap_or_else(|err| {
                panic!("Unable to parse file: {err}");
            });
        Grid::build(&file_contents.split_contents)
    }

    /// Test conversion between 1d and 2d indices
    #[test]
    fn test_gridmap_methods() {
        let grid = test_setup();

        assert_eq!(grid.idx_to_xy(0), (0, 0));
        assert_eq!(grid.idx_to_xy(1), (1, 0));
        assert_eq!(grid.idx_to_xy(4), (4, 0));
        assert_eq!(grid.idx_to_xy(5), (0, 1));
        assert_eq!(grid.idx_to_xy(21), (1, 4));
        assert_eq!(grid.idx_to_xy(24), (4, 4));

        assert_eq!(grid.xy_to_idx((0, 0)), 0);
        assert_eq!(grid.xy_to_idx((1, 0)), 1);
        assert_eq!(grid.xy_to_idx((4, 0)), 4);
        assert_eq!(grid.xy_to_idx((0, 1)), 5);
        assert_eq!(grid.xy_to_idx((1, 4)), 21);
        assert_eq!(grid.xy_to_idx((4, 4)), 24);
    }

    /// Test populating gridmap with forest heightmap
    #[test]
    fn test_input_gridmap() {
        let grid = test_setup();

        assert_eq!(grid.get((0, 0)), &3);
        assert_eq!(grid.get((4, 0)), &3);
        assert_eq!(grid.get((0, 1)), &2);
        assert_eq!(grid.get((4, 1)), &2);
        assert_eq!(grid.get((2, 2)), &3);
        assert_eq!(grid.get((4, 4)), &0);
    }

    #[test]
    fn test_count_trees() {
        let grid = test_setup();

        assert_eq!(grid.count_trees((0, 0), Direction::RIGHT), 2);
        assert_eq!(grid.count_trees((2, 0), Direction::DOWN), 2);
        assert_eq!(grid.count_trees((4, 0), Direction::LEFT), 2);

        assert_eq!(grid.count_trees((3, 4), Direction::UP), 1);
        assert_eq!(grid.count_trees((4, 4), Direction::LEFT), 2);
    }

    #[test]
    fn test_count_forest() {
        let grid = test_setup();

        let mut total_tree_count: u32 = 0;

        let mut hashset: HashSet<usize> = HashSet::new();

        // Left and right sides
        for i in 0..grid.height {
            total_tree_count +=
                grid.count_trees_without_duplicates((0, i), Direction::RIGHT, &mut hashset);
            total_tree_count += grid.count_trees_without_duplicates(
                (grid.width - 1, i),
                Direction::LEFT,
                &mut hashset,
            );
        }

        // Top and bottom sides
        for j in 0..grid.width {
            total_tree_count +=
                grid.count_trees_without_duplicates((j, 0), Direction::DOWN, &mut hashset);
            total_tree_count += grid.count_trees_without_duplicates(
                (j, grid.height - 1),
                Direction::UP,
                &mut hashset,
            );
        }

        assert_eq!(total_tree_count, 21);
    }

    #[test]
    fn test_scenic_spots() {
        let grid = test_setup();

        assert_eq!(grid.get_scenic_score(7), 4);
        assert_eq!(grid.get_scenic_score(17), 8);
        assert_eq!(grid.get_scenic_score(16), 1);
        assert_eq!(grid.get_scenic_score(16), 1);
    }

    #[test]
    fn test_max_scenic_score() {
        let grid = test_setup();

        let mut max_scenic_score = 0;

        for i in 0..(grid.height * grid.width) {
            let scenic_score = grid.get_scenic_score(i);
            if scenic_score >= max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }

        assert_eq!(max_scenic_score, 8);
    }
}

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Grid {
    pub origin: &'static str,
    cells: Vec<u8>,
    pub height: usize,
    pub width: usize,
}

impl Grid {
    pub fn build(file_contents: &Vec<String>) -> Grid {
        let mut cells: Vec<u8> = Vec::new();

        let height: usize = file_contents.len();
        let width: usize = file_contents[0].chars().count();

        for lines in file_contents {
            for number in lines.chars() {
                cells.push(number.to_digit(10).unwrap() as u8);
            }
        }

        Grid {
            origin: "top_left",
            cells: cells,
            height: height,
            width: width,
        }
    }

    /// Convert from 1D index to 2D (X,Y) position
    pub fn idx_to_xy(&self, idx: usize) -> (usize, usize) {
        // Origin is Top left,
        // Positive Y direction to the bottom,
        // Positive X direction to the right,

        (idx % self.width, idx / self.width)
    }

    /// Convert from 2D (X,Y) to 1D index
    pub fn xy_to_idx(&self, xy: (usize, usize)) -> usize {
        xy.1 * self.width + xy.0
    }

    /// Get immutable cell reference
    pub fn get(&self, xy: (usize, usize)) -> &u8 {
        &self.cells[self.xy_to_idx(xy)]
    }

    pub fn count_trees_without_duplicates(
        &self,
        origin: (usize, usize),
        direction: Direction,
        hashset: &mut HashSet<usize>,
    ) -> u32 {
        let mut step_idx: i16 = 0;
        let mut length: i16 = 0;

        // Try implementing the direction stepping here
        match direction {
            Direction::UP => {
                step_idx = -(self.width as i16);
                length = i16::try_from(origin.1).unwrap();
            }
            Direction::DOWN => {
                step_idx = self.width as i16;
                length = i16::try_from(self.height - origin.1).unwrap();
            }
            Direction::LEFT => {
                step_idx = -1;
                length = i16::try_from(origin.0).unwrap();
            }
            Direction::RIGHT => {
                step_idx = 1;
                length = i16::try_from(self.width - origin.0).unwrap();
            }
        }

        let mut cur_idx: i16 = self.xy_to_idx(origin) as i16;

        let mut max_height_seen: i8 = -1;
        let mut tree_count: u32 = 0;

        for _ in 0..length {
            let height = self.cells[usize::try_from(cur_idx).unwrap()] as i8;

            if height > max_height_seen {
                // println!("Saw tree {} of height {}", cur_idx, height);
                max_height_seen = height;
                if hashset.insert(cur_idx as usize) {
                    tree_count += 1;
                }
                // Tree is seen
            } else {
                // println!("Didnt see tree {} of height {}", cur_idx, height);
                // Tree is not seen
            }
            cur_idx += step_idx;
        }

        tree_count
    }

    pub fn count_trees(&self, origin: (usize, usize), direction: Direction) -> u32 {
        let step_idx;
        let length;

        println!("Direction: {:?}", direction);

        // Try implementing the direction stepping here
        match direction {
            Direction::UP => {
                step_idx = -(self.width as i16);
                length = i16::try_from(origin.1).unwrap();
            }
            Direction::DOWN => {
                step_idx = self.width as i16;
                length = i16::try_from(self.height - origin.1 - 1).unwrap();
            }
            Direction::LEFT => {
                step_idx = -1;
                length = i16::try_from(origin.0).unwrap();
            }
            Direction::RIGHT => {
                step_idx = 1;
                length = i16::try_from(self.width - origin.0 - 1).unwrap();
            }
        }

        let mut cur_idx: i16 = self.xy_to_idx(origin) as i16;

        let mut max_height_seen: i8 = -1;
        let mut tree_count: u32 = 0;

        for _ in 0..length {
            let height = self.cells[usize::try_from(cur_idx).unwrap()] as i8;

            if height > max_height_seen {
                max_height_seen = height;

                // Tree is seen
                tree_count += 1;
                // println!("  Saw tree {} Max {}", height, max_height_seen);
            } else {
                // Tree is not seen
                // println!("  Didn't see tree {}, Max {}", height, max_height_seen);
            }

            cur_idx += step_idx;
        }

        tree_count
    }

    pub fn count_trees_scenic(&self, origin: (usize, usize), direction: Direction) -> u32 {
        let step_idx;
        let length;

        println!("Direction: {:?}", direction);

        // Try implementing the direction stepping here
        match direction {
            Direction::UP => {
                step_idx = -(self.width as i16);
                length = i16::try_from(origin.1).unwrap();
            }
            Direction::DOWN => {
                step_idx = self.width as i16;
                length = i16::try_from(self.height - origin.1 - 1).unwrap();
            }
            Direction::LEFT => {
                step_idx = -1;
                length = i16::try_from(origin.0).unwrap();
            }
            Direction::RIGHT => {
                step_idx = 1;
                length = i16::try_from(self.width - origin.0 - 1).unwrap();
            }
        }

        let mut cur_idx: i16 = self.xy_to_idx(origin) as i16;
        let origin_height = self.cells[usize::try_from(cur_idx).unwrap()] as i8;

        let mut max_height_seen: i8 = -1;
        let mut tree_count: u32 = 0;

        // Don't start from the current tree
        cur_idx += step_idx;

        for _ in 0..length {
            let height = self.cells[usize::try_from(cur_idx).unwrap()] as i8;

            // If tree height is more then max height seen, it can be seen
            if height >= max_height_seen {
                max_height_seen = height;
                tree_count += 1;
                println!("  Saw tree {} Max {}", height, max_height_seen);
            } else if max_height_seen < origin_height && height < max_height_seen {
                tree_count += 1;
                println!("  Saw tree {} Max {}", height, max_height_seen);
            } else {
                println!("  Didn't see tree {}, Max {}", height, max_height_seen);
            }

            //Nothing more to see as soon as max height matches origin tree height
            if max_height_seen >= origin_height {
                println!("  Nothing more to see!");
                break;
            }

            cur_idx += step_idx;
        }
        tree_count
    }

    pub fn get_scenic_score(&self, idx: usize) -> u32 {
        let mut viewing_dist_arr: [u32; 4] = [0; 4];

        println!("======{}=======", idx);

        viewing_dist_arr[0] = self.count_trees_scenic(self.idx_to_xy(idx), Direction::UP);
        viewing_dist_arr[1] = self.count_trees_scenic(self.idx_to_xy(idx), Direction::LEFT);
        viewing_dist_arr[2] = self.count_trees_scenic(self.idx_to_xy(idx), Direction::DOWN);
        viewing_dist_arr[3] = self.count_trees_scenic(self.idx_to_xy(idx), Direction::RIGHT);

        let scenic_score = viewing_dist_arr.iter().fold(1, |acc, x| acc * x);

        println!(
            "Scenic score: {} = {} * {} * {} * {}",
            scenic_score,
            viewing_dist_arr[0],
            viewing_dist_arr[1],
            viewing_dist_arr[2],
            viewing_dist_arr[3]
        );

        println!("=============");

        scenic_score
    }
}
