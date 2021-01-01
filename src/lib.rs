use std::collections::HashMap;

#[derive(Debug)]
pub struct Array<T> {
    data: HashMap<u64, T>,
    length: u64,
}

impl<T> Array<T> {
    /// Creates an empty `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_data_structures_and_algorithms::Array;
    ///
    /// let mut array: Array<&str> = Array::new();
    /// ```
    pub fn new() -> Array<T> {
        Array {
            data: HashMap::new(),
            length: 0,
        }
    }

    /// Returns a Option reference to the value corresponding to the index.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_data_structures_and_algorithms::Array;
    ///
    /// let mut array: Array<&str> = Array::new();
    /// array.push("foo");
    /// assert_eq!(array.get(0), Some(&"foo"));
    /// assert_eq!(array.get(2), None);
    /// ```
    pub fn get(&mut self, index: u64) -> Option<&T> {
        self.data.get(&index)
    }

    pub fn push(&mut self, item: T) -> u64 {
        self.data.insert(self.length, item);
        self.length = self.length + 1;
        self.length
    }

    pub fn pop(&mut self) -> Option<T> {
        let last_item_index = self.length - 1;
        let item = self.data.remove(&last_item_index);
        self.length = last_item_index;
        item
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
