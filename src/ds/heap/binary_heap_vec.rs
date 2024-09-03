use super::Heap;

#[derive(Default)]
pub struct BinaryHeapVec<T: Ord + Clone> {
    data: Vec<T>,
}

impl<T: Ord + Clone> BinaryHeapVec<T> {
    pub fn new() -> Self {
        BinaryHeapVec { data: vec![] }
    }

    /// Bubble up element at given index
    fn sift_up(&mut self, index: usize) {
        let mut child_index = index;
        let child_value = self.data[index].clone();

        // While we are not at the top
        while child_index > 0 {
            // Find index of my parent
            let parent_index = (child_index - 1) / 2;

            // If child is larger or equal to the parent, we stop
            if child_value >= self.data[parent_index] {
                break;
            }

            // Otherwise, we override data of child with the data of the parent
            self.data[child_index] = self.data[parent_index].clone();
            child_index = parent_index;
        }

        self.data[child_index] = child_value;
    }

    fn sift_down(&mut self, index: usize) {
        let mut parent_index = index;
        let count = self.data.len();
        let parent_value = self.data[parent_index].clone();

        while parent_index < count / 2 {
            let mut child_index = 2 * parent_index + 1;

            if child_index < count - 1 && self.data[child_index] > self.data[child_index + 1] {
                child_index += 1;
            }

            if parent_value <= self.data[child_index] {
                break;
            }

            self.data[parent_index] = self.data[child_index].clone();
            parent_index = child_index;
        }

        self.data[parent_index] = parent_value;
    }
}

impl<T: Ord + Clone> Heap<T> for BinaryHeapVec<T> {
    fn push(&mut self, value: T) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        if self.data.len() == 1 {
            let popped_value = self.data.pop();
            return popped_value;
        }

        let count = self.data.len();
        self.data.swap(0, count - 1);

        let popped_value = self.data.pop();
        self.sift_down(0);

        popped_value
    }

    fn peek(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }

        Some(&self.data[0])
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data.clear()
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::heap::Heap;

    use super::BinaryHeapVec;

    #[test]
    fn test_basic_heap() {
        let mut heap = BinaryHeapVec::<i32>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), None);

        heap.push(4);
        heap.push(1);
        heap.push(3);
        heap.push(2);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap() {
        // Create an instance of LeftistHeap that implements Heap
        let mut heap: BinaryHeapVec<i32> = BinaryHeapVec::new();

        // Test inserting elements
        for i in 1..=100 {
            heap.push(i);
        }

        // Test the smallest element in the heap
        assert_eq!(heap.peek(), Some(&1));

        // Test inserting smaller elements after creating the heap
        for i in -100..=0 {
            heap.push(i);
        }

        // Test the smallest element in the heap
        assert_eq!(heap.peek(), Some(&-100));

        // Test popping all elements from the heap
        for i in -100..=100 {
            assert_eq!(heap.pop(), Some(i));
        }

        // Test heap is empty
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }
}
