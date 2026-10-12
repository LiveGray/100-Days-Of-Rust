pub fn parse_miles(inp: &str) -> Vec<u32> {
    inp
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}

pub fn progress_days(inp: &Vec<u32>) -> u32 {
    let mut progress_days: u32 = 0;

    for i in 0..inp.len()-1{
        if &inp[i] < &inp[i+1] {
            progress_days += 1;
        }
    }

    progress_days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_miles() {
        assert_eq!(vec![1, 2, 3], parse_miles("1, 2, 3"));
    }

    #[test]
    fn miles_with_newline() {
        assert_eq!(vec![1, 2, 3], parse_miles("1, 2, 3\n"));
    }

    #[test]
    fn miles_with_space() {
        assert_eq!(vec![1, 2, 3], parse_miles("1, 2, 3 "));
    }

    #[test]
    fn no_progress() {
        assert_eq!(0, progress_days(&parse_miles("1, 1, 1")));
    }

    #[test]
    fn one_progress() {
        assert_eq!(1, progress_days(&parse_miles("1, 2, 1")));
    }
}
