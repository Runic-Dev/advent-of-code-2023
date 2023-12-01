use std::fs;

pub fn parse_callibration_number(input: &str) -> usize {
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
}
