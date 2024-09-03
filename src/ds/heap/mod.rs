pub mod binary_heap_vec;
pub mod binomial_heap;
pub mod leftist_heap;

pub trait Heap<T: Ord> {
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, element: T);
    fn size(&self) -> usize;

    fn clear(&mut self) {
        while self.pop() != None {}
    }
}

/// Insert a number of consecutive elements from 1 to n inclusive
pub fn insert_n_elements<H: Heap<i32>>(heap: &mut H, n: i32) {
    for i in 1..n {
        heap.push(i);
    }
}

/// Insert a number of elements from a vector
pub fn insert_n_vector_elements<H: Heap<i32>>(heap: &mut H, vec: &Vec<i32>) {
    for i in vec {
        heap.push(*i);
    }
}
