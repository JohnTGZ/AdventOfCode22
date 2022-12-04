//! # Advent of Code Common library
//! 
//! `aoc_common` contains common functions used by all the AOC puzzles to 
//!  make life easier.

use std::error::Error;
use std::fs;

pub struct FileContents {
    pub input_filepath: String,
    pub contents: String,
    pub split_contents: Vec<String>,
}

impl FileContents {
    /// Parse the input file to retrieve each line and collect it into a vector
    pub fn build(input_filepath: &str, split_delim: &str) -> Result<FileContents, Box<dyn Error>>
    {
        let contents = fs::read_to_string(input_filepath)?;

        let split_contents: Vec<String> = contents
                                .split(split_delim)
                                .map(|s| s.to_string())
                                .collect();

        Ok(FileContents{
            input_filepath: input_filepath.to_string(),
            contents: contents,
            split_contents: split_contents,
        })
    }

    pub fn split_line<'a>(input: &'a str, split_delim: &str) -> Vec<&'a str> 
    {
        input.split(split_delim).collect()
    } 

    pub fn split_into_i32(input: & str, split_delim: &str) -> Vec<i32> 
    {
        let str_vec: Vec<&str> = input.split(split_delim).collect();

        str_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect()
    } 

}
