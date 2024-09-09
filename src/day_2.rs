use std::iter::zip;
use itertools::Itertools;

use crate::file_utilities::read_lines;

fn parse_data(file_path: String) -> Vec<String> {
    read_lines(file_path)
        .into_iter()
        .collect()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> String {
    match part {
        1 => part_1(file_path).to_string(),
        2 => part_2(file_path),
        _ => panic!("... nope.")
    }
}

fn part_1(file_path: String) -> i32 {
    let box_ids = parse_data(file_path);
    let mut two_of_letter = 0;
    let mut three_of_letter = 0;

    for box_id in box_ids.into_iter()
    {
        let counts = box_id.chars().counts();
        let counts_of_counts = counts.values().copied().counts();

        if counts_of_counts.contains_key(&2) {
            two_of_letter += 1;
        }

        if counts_of_counts.contains_key(&3) {
            three_of_letter += 1;
        }
    }

    two_of_letter * three_of_letter
}

fn part_2(file_path: String) -> String {
    let box_ids = parse_data(file_path);

    for box_id_1 in box_ids.iter() {
        for box_id_2 in box_ids.iter() {
            let same: Vec<char> = zip(box_id_1.chars(), box_id_2.chars())
                .filter(|(char_1, char_2)| char_1 == char_2)
                .map(|(char_1, _)| char_1)
                .collect();

            if same.len() == box_id_1.len() - 1
            {
                return same.iter().collect();
            }
        }
    }

    "No solution found!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::file_utilities::get_file_path;

    #[rstest]
    #[case(true, Some(String::from("1")), 12)]
    #[case(false, None, 9633)]
    fn test_part_1(#[case] is_test: bool, #[case] suffix: Option<String>, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 2, suffix)));
    }

    #[rstest]
    #[case(true, Some(String::from("2")), "fgij")]
    #[case(false, None, "lujnogabetpmsydyfcovzixaw")]
    fn test_part_2(#[case] is_test: bool, #[case] suffix: Option<String>, #[case] expected: String) {
        assert_eq!(expected, part_2(get_file_path(is_test, 2, suffix)));
    }
}