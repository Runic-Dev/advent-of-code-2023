use std::{fs, collections::HashMap};


pub fn get_callibration_sum() -> usize {
    let file_as_string = read_calibration_file();
    let lines = file_as_string.lines();

    lines.map(|line| {
        parse_callibration_number(line)
    }).reduce(|acc, value| acc + value).unwrap()
}

fn parse_callibration_number(input: &str) -> usize {
    let joined_number_str = format!("{}{}", parse_first_number(input), parse_last_number(input));
    match joined_number_str.parse::<usize>() {
        Ok(joined_number) => joined_number,
        Err(err) => panic!("{}", err),
    }
}

fn read_calibration_file() -> String {
    match fs::read_to_string("input.txt") {
        Ok(read_file) => read_file,
        Err(err) => panic!("Couldn't read from file: {}", err),
    }
}

fn parse_first_number(input: &str) -> char {
    for char in input.chars() {
        if char.is_numeric() {
            return char;
        }
    }

    panic!("No number found");
}

fn parse_last_number(input: &str) -> char {
    let rev_str = input.chars().rev().collect::<String>();
    parse_first_number(&rev_str)
}

fn clean_string(input: String) -> String {
    let mut result = input.clone();
    let dict = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut matches: Vec<(usize, &str)> = vec![];

    for (word_number, _) in dict.iter() {
        let word_matches = input.match_indices(word_number);
        matches.extend(word_matches);
    }

    matches.sort_by(|x, y| x.0.cmp(&y.0));

    let first = matches.first().unwrap();
    let last = matches.last().unwrap();
    let first_replacement = dict.get(first.1).unwrap();
    let last_replacement = dict.get(last.1).unwrap();

    result = result.replacen(first.1, first_replacement, 1);
    result = result.replacen(last.1, last_replacement, 1);

    result
}

#[cfg(test)]
mod calibration_parser_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn return_1_when_first_number_is_1() {
        let input = String::from("abc1def");
        let first_number = parse_first_number(&input);
        assert_eq!('1', first_number);
    }

    #[test]
    fn return_2_when_first_number_is_2() {
        let input = String::from("abc2def");
        let first_number = parse_first_number(&input);
        assert_eq!('2', first_number);
    }

    #[test]
    fn return_3_when_first_number_is_3() {
        let input = String::from("abc3def");
        let first_number = parse_first_number(&input);
        assert_eq!('3', first_number);
    }

    #[test]
    fn return_4_when_first_number_is_4() {
        let input = String::from("abc4def");
        let first_number = parse_first_number(&input);
        assert_eq!('4', first_number);
    }

    #[test]
    fn return_1_when_last_number_is_1() {
        let input = String::from("abc4def1aa");
        let last_number = parse_last_number(&input);
        assert_eq!('1', last_number);
    }

    #[rstest]
    #[case(String::from("abc149dfsdf299fds2"), 12)]
    #[case(String::from("abc349dfsdf299fds2"), 32)]
    #[case(String::from("abc549dfsdf299fds2"), 52)]
    #[case(String::from("abc849dfsdf299fds2"), 82)]
    #[case(String::from("abc149dfsdf299fds4"), 14)]
    #[case(String::from("abc949dfsdf299fds2"), 92)]
    fn return_number_composed_of_first_and_last_number(#[case] input: String, #[case] expected: usize) {
        let joined_number = parse_callibration_number(&input);
        assert_eq!(expected, joined_number);
    }

    #[test]
    fn clean_numbers_as_words() {
        let input = String::from("abcone2threexyz");
        let result = clean_string(input);
        assert_eq!(result, "abc123xyz")
    }

}
