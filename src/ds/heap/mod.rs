use rand::Rng;

pub mod leftist_heap;
pub mod binary_heap_vec;

pub trait Heap<T : Ord> {
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, element: T);
    fn size(&self) -> usize;

    fn clear(&mut self) {
        while self.pop() != None {}
    }
}

pub fn insert_n_elements<H : Heap<i32>>(heap: &mut H, n: i32) {
    for i in 1..n {
        heap.push(i);
    }
}

pub fn generate_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for _ in 0 .. n {
        vec.push(rng.gen_range(0..100_000_000));
    }
    
    vec
}

pub fn insert_n_vector_elements<H : Heap<i32>>(heap: &mut H, vec: &Vec<i32>) {
    for i in vec {
        heap.push(*i);
    }
}