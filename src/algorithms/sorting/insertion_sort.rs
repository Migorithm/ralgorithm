#![allow(dead_code)]

// sort a subset of the array and expanding this sorted range until the entire array is in order.
fn insertion_sort<T>(array: &mut [T])
where
    T: Ord + Copy,
{
    let length = array.len();

    // starts at the first unsorted element, i=1
    for i in 1..length {
        let current = *array.get(i).unwrap();

        let mut j = (i - 1) as i32;

        while (j >= 0) && array[j as usize] > current {
            // move elements of array[0..i-1], that are greater than key, to one position ahead of their current position
            array[(j + 1) as usize] = array[j as usize];
            j -= 1;
        }

        array[(j + 1) as usize] = current;
    }
}

#[test]
fn test_insertion_sort() {
    let mut array = [4, 3, 2, 1];
    insertion_sort(&mut array);
    assert_eq!(array, [1, 2, 3, 4]);

    let mut array2 = [1, 2, 32, 3, 2, 123, 65, 7865, 65];
    insertion_sort(&mut array2);
    assert_eq!(array2, [1, 2, 2, 3, 32, 65, 65, 123, 7865]);
}
