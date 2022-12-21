use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_start_packet() {
        let test_str_list = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let packet_size: usize = 4;
        let mut start_packet_idx: [usize; 5] = [packet_size; 5];
        let mut start_packets: [&str; 5] = [""; 5];

        let mut char_set: HashSet<u8> = HashSet::new();

        for (str_idx, test_str) in test_str_list.iter().enumerate() {
            //Test for 4 characters at a time, from 0..test_str_1.len()-3
            for s_idx in 0..test_str.len() - packet_size + 1 {
                char_set.clear();

                // let mut cur_idx = s_idx;
                for character in &test_str.as_bytes()[s_idx..s_idx + packet_size] {
                    // println!("idx {} -> Adding {}", cur_idx, character.clone() as char);
                    // cur_idx += 1;
                    char_set.insert(*character);
                }
                // display_char_set(&char_set);

                if char_set.len() == packet_size {
                    start_packet_idx[str_idx] = s_idx + packet_size;

                    start_packets[str_idx] = &test_str[s_idx..(s_idx + packet_size)];
                    break;
                }
            }
        }

        assert_eq!(7, start_packet_idx[0]);
        assert_eq!(5, start_packet_idx[1]);
        assert_eq!(6, start_packet_idx[2]);
        assert_eq!(10, start_packet_idx[3]);
        assert_eq!(11, start_packet_idx[4]);

        assert_eq!("jpqm", start_packets[0]);
        assert_eq!("vwbj", start_packets[1]);
        assert_eq!("pdvj", start_packets[2]);
        assert_eq!("rfnt", start_packets[3]);
        assert_eq!("zqfr", start_packets[4]);
    }
}

pub fn display_char_set(char_set: &HashSet<u8>) -> () {
    let mut display_str = "".to_string();
    for character in char_set {
        display_str.push(character.clone() as char);
    }
    // println!("{}", display_str);
}
