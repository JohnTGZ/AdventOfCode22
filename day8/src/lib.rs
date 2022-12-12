use aoc_common::FileContents;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_setup() -> Grid{
        let test_input_filepath = "../input/day8/test_input.txt";
        let split_delim = "\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim, -1, -1)
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

        assert_eq!(grid.get((0,0)), &3);
        assert_eq!(grid.get((4,0)), &3);
        assert_eq!(grid.get((0,1)), &2);
        assert_eq!(grid.get((4,1)), &2);
        assert_eq!(grid.get((2,2)), &3);
        assert_eq!(grid.get((4,4)), &0);
    }

    // #[test]
    // fn test_count_trees() {
    //     let grid = test_setup();
    //     let mut hashset: HashSet<usize> = HashSet::new();

    //     assert_eq!(grid.count_trees((0,0), Direction::RIGHT, &mut hashset), 2);
    //     assert_eq!(grid.count_trees((2,0), Direction::DOWN, &mut hashset), 2);
    //     assert_eq!(grid.count_trees((4,0), Direction::LEFT, &mut hashset), 2);

    //     assert_eq!(grid.count_trees((3,4), Direction::UP, &mut hashset), 1);
    //     assert_eq!(grid.count_trees((4,4), Direction::LEFT, &mut hashset), 2);

    // }
    
    #[test]
    fn test_count_forest(){
        let grid = test_setup();

        let mut total_tree_count: u32 = 0;

        let mut hashset: HashSet<usize> = HashSet::new();

        // Left and right sides
        for i in 0..grid.height {
            total_tree_count += grid.count_trees((0, i), Direction::RIGHT, &mut hashset);
            total_tree_count += grid.count_trees((grid.width-1, i), Direction::LEFT, &mut hashset);
        }

        // Top and bottom sides
        for j in 0..grid.width {
            total_tree_count += grid.count_trees((j, 0), Direction::DOWN, &mut hashset);
            total_tree_count += grid.count_trees((j, grid.height-1), Direction::UP, &mut hashset);
        }

        for num in &hashset {
            println!("{}", num);
        }

        assert_eq!(total_tree_count, 21);
    }

}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Grid{
    pub origin: &'static str,
    cells: Vec<u8>,
    pub height: usize,
    pub width: usize,
}

impl Grid{
    pub fn build(file_contents: &Vec<String>) -> Grid {
        let mut cells: Vec<u8> = Vec::new();

        let height: usize = file_contents.len();
        let width: usize = file_contents[0].chars().count();

        for lines in file_contents{
            for number in lines.chars(){
                cells.push(number.to_digit(10).unwrap() as u8);
            }
        }

        Grid{
            origin: "top_left",
            cells: cells,
            height: height,
            width: width, 
        }
    }

    /// Convert from 1D index to 2D (X,Y) position
    pub fn idx_to_xy(&self, idx: usize) -> (usize, usize){
        // Origin is Top left, 
        // Positive Y direction to the bottom, 
        // Positive X direction to the right,

        (idx % self.width, idx/self.width)
    }

    /// Convert from 2D (X,Y) to 1D index
    pub fn xy_to_idx(&self, xy: (usize, usize)) -> usize{
        xy.1 * self.width + xy.0
    }

    /// Get immutable cell reference 
    pub fn get(&self, xy: (usize, usize)) -> &u8{
        &self.cells[self.xy_to_idx(xy)]
    }

    pub fn count_trees(&self, origin: (usize, usize), direction: Direction, hashset: &mut HashSet<usize>) -> u32 {
        let mut step_idx: i16 = 0;
        let mut length = 0;

        // UP = (0, -1),
        // DOWN = (0, 1),
        // LEFT = (-1, 0),
        // RIGHT = (1, 0),
        match direction {
            Direction::UP => { 
                step_idx = -(self.width as i16);
                length = self.height;
            },
            Direction::DOWN => {
                step_idx = self.width as i16;
                length = self.height;
            },
            Direction::LEFT => {
                step_idx = -1;
                length = self.width;
            },
            Direction::RIGHT => {
                step_idx = 1;
                length = self.width;
            },
        }

        let mut cur_idx: i16 = self.xy_to_idx(origin) as i16;

        let mut max_height_seen: i8 = -1;
        let mut tree_count:u32 = 0;

        for _ in 0..length {
            let height = self.cells[usize::try_from(cur_idx).unwrap()] as i8;

            if height > max_height_seen {
                println!("Saw tree {} of height {}", cur_idx, height);
                max_height_seen = height;
                if hashset.insert(cur_idx as usize) {
                    tree_count += 1;
                }
                // Tree is seen
            }
            else {
                println!("Didnt see tree {} of height {}", cur_idx, height);
                // Tree is not seen
            }
            cur_idx += step_idx;
        }   

        tree_count
    }

}   



