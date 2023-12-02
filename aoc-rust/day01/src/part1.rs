use rstest::*;

pub fn process(input: &str) -> i32 {
    let mut result: i32 = 0;

    for line in input.split('\n') {
        let mut lign_result: String = "".into();

        for c in line.chars() {
            if c.is_digit(10) {
                lign_result.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                lign_result.push(c);
                break;
            }
        }

        let parsed_result = lign_result.parse::<i32>();
        match parsed_result {
            Err(_) => {
                continue;
            },
            Ok(parsed_number) => {
                result += parsed_number;
            }
        }
    }
    return result;
}

mod tests {
    use super::*;

    #[rstest]
    fn test_process() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!(142, process(input));
    }
}
