struct DoubleArray<T: Ord + Copy> {
    data: Box<[T]>,
    len: usize,
}

impl<T: Ord + Copy + std::fmt::Debug + Default> DoubleArray<T> {
    fn push(&mut self, value: T) {
        if self.len() < self.capacity() {
            self.data[self.len] = value;
            self.len += 1;
        } else {
            let new_cap = self.capacity() * 2;
            let mut v = Vec::with_capacity(new_cap);
            v.resize(new_cap, T::default());

            v[..self.len()].clone_from_slice(&self.data[..self.len()]);

            v[self.len] = value;
            self.len += 1;
            self.data = v.into_boxed_slice();
        }
    }

    fn len(&self) -> usize {
        self.len
    }
    fn capacity(&self) -> usize {
        self.data.len()
    }
}

#[test]
fn test_double_array() {
    // Create a new DoubleArray with an initial capacity of 2
    let mut array = DoubleArray {
        data: Box::new([0; 2]),
        len: 0,
    };

    // Push elements to the array
    array.push(1);
    array.push(2);

    assert_eq!(array.len(), 2);
    assert_eq!(array.capacity(), 2);

    // Trigger the doubling of capacity
    array.push(3);

    assert_eq!(array.len(), 3);
    assert_eq!(array.capacity(), 4);

    array.push(4);
    array.push(5);

    assert_eq!(array.len(), 5);
    assert_eq!(array.capacity(), 8);

    // Access elements
    for i in 0..array.len() {
        println!("Element at {}: {}", i, array.data[i]);
    }
}
