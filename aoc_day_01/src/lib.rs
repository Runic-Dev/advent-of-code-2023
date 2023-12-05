pub fn extract_number(file_input: String) -> usize {
    file_input
        .lines()
        .map(|line| {
            let mut digits: Vec<usize> = vec![];
            line.chars().enumerate().for_each(|(i, char)| {
                if char.is_ascii_digit() {
                    digits.push(format!("{}", char).parse::<usize>().unwrap());
                    return;
                }
                [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .enumerate()
                .for_each(|(char_i, word_number)| {
                    if let Some(segment) = line.get(i..) {
                        if segment.starts_with(word_number) {
                            digits.push(char_i + 1);
                        }
                    }
                })
            });
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .reduce(|acc, value| acc + value)
        .unwrap()
}

#[cfg(test)]
pub mod test {
    use rstest::rstest;

    use crate::extract_number;

    #[rstest]
    #[case("abcnineabc", 99)]
    #[case("abceightwo", 82)]
    #[case("2abskjfdfhdkkl2", 22)]
    #[case("xxabonefdfsevenseven", 17)]
    fn test_cases(#[case] input: String, #[case] expected: usize) {
        let result = extract_number(input);
        assert_eq!(result, expected);
    }
}
