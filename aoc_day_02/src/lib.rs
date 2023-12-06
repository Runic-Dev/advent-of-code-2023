pub fn parse_color_amounts(input: String) -> GameRow {
    let split = input.split_once(": ").unwrap();
    let id = parse_game_id(split.0.to_string());
    let mut rounds: Vec<Vec<(usize, String)>> = vec![];
    parse_rounds(split.1.to_string()).iter().for_each(|round| {
        let parsed_amount_colors = parse_color_amount(round.to_string());
        rounds.push(parsed_amount_colors);
    });
    (id, rounds)
}

pub type Round = Vec<(usize, String)>;
pub type GameRow = (usize, Vec<Round>);

pub fn add_qualifying_ids(game_rows: Vec<GameRow>) -> (usize, u128) {
    let mut answer: usize = 0;
    let mut power: u128 = 0;
    game_rows.into_iter().for_each(|row| {
        let red_number = get_largest_number_for(&row, "red");
        let green_number = get_largest_number_for(&row, "green");
        let blue_number = get_largest_number_for(&row, "blue");

        let this_power = red_number * green_number * blue_number;

        power += this_power as u128;

        if !game_is_impossible(&row) {
            answer += row.0
        }
    });
    (answer, power)
}

fn game_is_impossible(row: &(usize, Vec<Vec<(usize, String)>>)) -> bool {
    let impossible = row.1.iter().any(|round| {
        round.iter().any(|color_amount| {
            (color_amount.1 == "red" && color_amount.0 > 12)
                || (color_amount.1 == "green" && color_amount.0 > 13)
                || (color_amount.1 == "blue" && color_amount.0 > 14)
        })
    });
    impossible
}

fn get_largest_number_for(row: &GameRow, color: &str) -> usize {
    let mut red_numbers = row
        .1
        .iter()
        .flatten()
        .filter(|entry| entry.1.contains(color))
        .map(|(amount, _)| amount)
        .collect::<Vec<&usize>>();
    red_numbers.sort();
    *red_numbers.last().cloned().unwrap()
}

fn parse_color_amount(input: String) -> Round {
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

fn parse_rounds(input: String) -> Vec<String> {
    input.split("; ").map(|x| x.to_string()).collect()
}

#[cfg(test)]
pub mod test {
    use super::parse_color_amount;
    use super::parse_game_id;
    use super::parse_rounds;
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

    #[rstest]
    #[case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", vec!["3 blue, 4 red".to_string(), "1 red, 2 green, 6 blue".to_string(), "2 green".to_string()])]
    fn parse_rounds_from_string(#[case] input: String, #[case] expected: Vec<String>) {
        let result = parse_rounds(input);
        assert_eq!(result, expected);
    }
}
