use regex::Regex;

pub fn parse_ints(s: &str) -> Vec<i32> {
    let re = Regex::new(r"[+-]?\d+").unwrap();
    re.find_iter(s)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_ints() {
        assert_eq!(parse_ints(""), vec![]);
        assert_eq!(parse_ints("1 2 3 -10, 0"), vec![1, 2, 3, -10, 0]);
    }
}
