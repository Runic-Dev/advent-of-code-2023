fn parse_color_amount(input: String) -> Vec<(usize, String)> {
    input
        .split(", ")
        .map(|color_amount_str| {
            let (amount, color) = color_amount_str.split_once(' ').unwrap();
            (amount.parse::<usize>().unwrap(), color.to_string())
        })
        .collect::<Vec<(usize, String)>>()
}

fn parse_game_id(input: String) -> usize {
    input
        .split(' ')
        .last()
        .expect("Invalid format for Game Id info")
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
pub mod test {
    use super::parse_color_amount;
    use super::parse_game_id;
    use rstest::rstest;

    #[rstest]
    #[case("3 blue, 5 red", vec![
        (3, "blue".to_string()),
        (5, "red".to_string())
    ])]
    #[case("5 red, 9 blue, 22 green", vec![
        (5, "red".to_string()),
        (9, "blue".to_string()),
        (22, "green".to_string())
    ])]
    fn parse_color_and_amount_from_string(
        #[case] input: String,
        #[case] expected: Vec<(usize, String)>,
    ) {
        let parsed_info: Vec<(usize, String)> = parse_color_amount(input);
        let first_tuple = parsed_info.first().unwrap();
        let last_tuple = parsed_info.last().unwrap();
        assert_eq!(first_tuple.0, expected.first().unwrap().0);
        assert_eq!(first_tuple.1, expected.first().unwrap().1);
        assert_eq!(last_tuple.0, expected.last().unwrap().0);
        assert_eq!(last_tuple.1, expected.last().unwrap().1);
    }

    #[rstest]
    #[case("Game 1", 1)]
    #[case("Game 2", 2)]
    #[case("Game 3", 3)]
    #[case("Game 45", 45)]
    #[case("Game 59", 59)]
    fn parse_game_ids_from_string(#[case] input: String, #[case] expected: usize) {
        let result = parse_game_id(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_rounds_from_string() {
        unimplemented!();
    }
}
