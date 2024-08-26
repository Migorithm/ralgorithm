pub fn binary_search<T: Ord + Copy>(arr: &[T], target: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (high + low) / 2;
        let midpoint = arr[mid];

        match midpoint.cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
        }
    }
    None
}

#[test]
fn test_binary_search() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(binary_search(&arr, 11), None);
    assert_eq!(binary_search(&arr, 5), Some(4));
}

#[test]
fn test_binary_search2() {
    let arr = [6, 10, 39, 49, 299, 300];
    assert_eq!(binary_search(&arr, 300), Some(5));
}
