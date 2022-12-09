//! # Advent of Code Common library
//! 
//! `aoc_common` contains common functions used by all the AOC puzzles to 
//!  make life easier.

use std::error::Error;
use std::fs;

use std::ops::{Range, RangeFrom, RangeBounds};

pub struct FileContents {
    pub input_filepath: String,
    pub contents: String,
    pub split_contents: Vec<String>,
}

impl FileContents {
    /// Parse the input file to retrieve each line and collect it into a vector
    // pub fn build(input_filepath: &str, split_delim: &str, lines_to_read: Option<Range<usize>>) -> Result<FileContents, Box<dyn Error>>

    // pub fn build<R>(input_filepath: &str, split_delim: &str, lines_to_read: Option<R>) 
    // -> Result<FileContents, Box<dyn Error>>
    // where 
    //     R: RangeBounds<usize>
    pub fn build(input_filepath: &str, split_delim: &str, start: i32, end: i32) 
    -> Result<FileContents, Box<dyn Error>>
    {
        let contents = fs::read_to_string(input_filepath)?;

        let num_lines = contents.lines().count();

        let start_line: usize = match start {
            -1 => 0,
            other => {
                other as usize
            },
        };

        let end_line: usize = match end {
            -1 => num_lines,
            other => {
                other as usize
            },
        };

        // eprintln!("Range of lines to read: {} -> {}", start_line, end_line);

        if start_line >= end_line {
            panic!("Start and end range for reading lines is the same");
        }

        let split_contents: Vec<String> = contents
                                .split(split_delim)
                                .take(end_line)
                                .skip(start_line)
                                .map(|s| s.to_string())
                                .collect();

        Ok(FileContents{
            input_filepath: input_filepath.to_string(),
            contents: contents,
            split_contents: split_contents,
        })
    }

    pub fn print(&self) -> () {
        for (idx, content) in self.split_contents.iter().enumerate(){
            println!("Line {idx}: {content}");
        }
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
