// O(n) - effecient for small data sets and unsorted data
pub fn sequential_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

// write binary search tree

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(sequential_search(&arr, &11), None);
        assert_eq!(sequential_search(&arr, &5), Some(4));
    }
}
