use std::collections::HashMap;

#[derive(Debug)]
pub struct Array<T> {
    data: HashMap<u64, T>,
    pub length: u64,
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

    /// The `push` method add elements to the end
    /// of an array and returns the new length of the array.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_data_structures_and_algorithms::Array;
    ///
    /// let mut array: Array<&str> = Array::new();
    /// array.push("foo");
    /// assert_eq!(array.length, 1);
    /// assert_eq!(array.push("bar"), 2);
    /// assert_eq!(array.get(0), Some(&"foo"));
    /// assert_eq!(array.get(1), Some(&"bar"));
    /// assert_eq!(array.get(2), None);
    /// ```
    pub fn push(&mut self, item: T) -> u64 {
        self.data.insert(self.length, item);
        self.length += 1;
        self.length
    }

    /// The `pop` method removes the last element from an array
    /// and returns that element. This method changes the length of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_data_structures_and_algorithms::Array;
    ///
    /// let mut array: Array<&str> = Array::new();
    /// array.push("foo");
    /// array.push("bar");
    /// array.push("x");
    /// assert_eq!(array.length, 3);
    /// assert_eq!(array.pop(), Some("x"));
    /// assert_eq!(array.length, 2);
    /// assert_eq!(array.pop(), Some("bar"));
    /// assert_eq!(array.pop(), Some("foo"));
    /// assert_eq!(array.length, 0);
    /// assert_eq!(array.pop(), None);
    /// assert_eq!(array.length, 0);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            let last_item_index = self.length - 1;
            let item = self.data.remove(&last_item_index);
            self.length = last_item_index;
            item
        } else {
            None
        }
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
