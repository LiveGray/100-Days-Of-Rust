use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn is_veg(inp: &str) -> bool {
    inp.contains("o") && !inp.contains("x")
}

pub fn check_skwrs(inp: &[&str]) -> [usize; 2] {
    let veg = inp.into_iter().map(|str| is_veg(str)).filter(|&b| b).count();
    let not_veg = inp.len() - veg;
    [veg, not_veg]

}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn veg() {
        assert_eq!(true, is_veg("--oo--o--oo--"));
    }

    #[test]
    fn not_veg() {
        assert_eq!(false, is_veg("--xo--o--xo--"));
    }

    #[test]
    fn count_veg() {
        assert_eq!([1,2], check_skwrs(&["-oox-xo-", "-oo-ooo-", "-xx-xo-"]))
    }

    #[test]
    fn check_veg() {
        assert_eq!([3,0], check_skwrs(&["-oo-o-", "-oo-ooo-", "-oo-o-"]))
    }
}
