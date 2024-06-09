// write a function that takes a text and a pattern and returns the index of the first occurrence of the pattern in the text
// if the pattern is not found, the function should return None
pub fn brute_force(text: &str, pattern: &str) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();

    // For the pattern to fit within the text, the last character of the pattern must not exceed the length of the text.
    // Therefore i + m - 1 < n must be true.
    // Simplifying, i < n - m + 1.
    for i in 0..(n - m + 1) {
        let mut j = 0;
        while j < m && text.chars().nth(i + j).unwrap() == pattern.chars().nth(j).unwrap() {
            j += 1;
        }
        if j == m {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let a = 0..10 - 5 + 1;
        println!("{:?}", a)
    }
    #[test]
    fn test_brute_force() {
        assert_eq!(brute_force("hello", "ll"), Some(2));
        assert_eq!(brute_force("hello", "el"), Some(1));
        assert_eq!(brute_force("hello", "he"), Some(0));
        assert_eq!(brute_force("hello", "lo"), Some(3));
        assert_eq!(brute_force("hello", "hello"), Some(0));
        assert_eq!(brute_force("hello", "world"), None);
    }
}
