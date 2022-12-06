use std::{fs, collections::HashSet};

pub struct Day6 {}

impl Day6 {
    pub fn run() {
        let filename = "src/day6/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let marker = Self::get_packet_marker(&data);

        println!("Part 1 - Packet marker is: {}", marker);

        // Part 2
        let marker = Self::get_msg_marker(&data);

        println!("Part 2 - Message index is: {}", marker);
    }

    fn get_marker(data: &str, group_size: usize) -> usize {
        let mut marker = 0;

        for i in 0..(data.len() - (group_size - 1)) {
            let group: HashSet<char> = data.chars().skip(i).take(group_size).collect();
            if group.len() == group_size {
                marker = i + group_size;
                break;
            }
        }
    
        marker
    }

    fn get_packet_marker(data: &str) -> usize {
        Self::get_marker(data, 4)
    }

    fn get_msg_marker(data: &str) -> usize {
        Self::get_marker(data, 14)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_marker() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker = Day6::get_packet_marker(data);
        assert_eq!(marker, 7);

        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = Day6::get_packet_marker(data);
        assert_eq!(marker, 5);

        let data = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = Day6::get_packet_marker(data);
        assert_eq!(marker, 6);

        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = Day6::get_packet_marker(data);
        assert_eq!(marker, 10);

        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = Day6::get_packet_marker(data);
        assert_eq!(marker, 11);
    }

    #[test]
    fn test_msg_marker() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker = Day6::get_msg_marker(data);
        assert_eq!(marker, 19);

        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = Day6::get_msg_marker(data);
        assert_eq!(marker, 23);

        let data = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = Day6::get_msg_marker(data);
        assert_eq!(marker, 23);

        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = Day6::get_msg_marker(data);
        assert_eq!(marker, 29);

        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = Day6::get_msg_marker(data);
        assert_eq!(marker, 26);
    }
}