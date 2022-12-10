use std::fs;
use std::collections::HashSet;

fn main() {
    let input_filepath = "input/day6/final_input.txt";
    let test_str = fs::read_to_string(input_filepath).expect("Unable to read file");

    let mut packet_size: usize = 4;
    let mut start_packet_idx = packet_size;
    let mut start_packets=  "";

    let mut char_set: HashSet<u8> = HashSet::new();
        
    //Test for 4 characters at a time, from 0..test_str_1.len()-3
    for s_idx in 0..test_str.len()-packet_size+1{
        char_set.clear();
        
        for character in &test_str.as_bytes()[s_idx..s_idx+packet_size]{
            char_set.insert(*character);
        }

        if char_set.len() == packet_size{
            start_packet_idx = s_idx + packet_size;

            start_packets = &test_str[s_idx..(s_idx + packet_size)];
            break;
        }
    }

    println!("Part 1, Index: {}, packet: {}", start_packet_idx, start_packets);


    // Same thing, but different packet size
    packet_size = 14; 

    for s_idx in 0..test_str.len()-packet_size+1{
        char_set.clear();
        
        for character in &test_str.as_bytes()[s_idx..s_idx+packet_size]{
            char_set.insert(*character);
        }

        if char_set.len() == packet_size{
            start_packet_idx = s_idx + packet_size;

            start_packets = &test_str[s_idx..(s_idx + packet_size)];
            break;
        }
    }

    println!("Part 2, Index: {}, packet: {}", start_packet_idx, start_packets);

}
