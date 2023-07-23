use super::Heap;

struct Node<T> {
    element: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    rank: usize,
}

impl<T: Ord> Node<T> {
    fn new(element: T) -> Node<T> {
        Node {
            element,
            left: None,
            right: None,
            rank: 0,
        }
    }
}

pub struct LeftistHeap<T> {
    root: Option<Box<Node<T>>>,
}


impl<T: Ord> LeftistHeap<T> {
    pub fn new() -> LeftistHeap<T> {
        LeftistHeap { root: None }
    }
    
    // fn merge(&mut self, mut other: LeftistHeap<T>) {
    //     self.root = Self::merge_nodes(self.root.take(), other.root.take());
    // }
    
    fn merge_nodes(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match (left, right) {
            (None, right) => right,
            (left, None) => left,
            (Some(mut left_node), Some(mut right_node)) => {
                if left_node.element <= right_node.element {
                    left_node.right = Self::merge_nodes(left_node.right.take(), Some(right_node));
                    Self::update_rank(&mut left_node);
                    Some(left_node)
                } else {
                    right_node.right = Self::merge_nodes(right_node.right.take(), Some(left_node));
                    Self::update_rank(&mut right_node);
                    Some(right_node)
                }
            }
        }
    }
    
    fn update_rank(node: &mut Box<Node<T>>) {
        let rank_left = node.left.as_ref().map_or(0, |left_node| left_node.rank + 1);
        let rank_right = node.right.as_ref().map_or(0, |right_node| right_node.rank + 1);
        node.rank = std::cmp::min(rank_left, rank_right);
        if rank_left < rank_right {
            std::mem::swap(&mut node.left, &mut node.right);
        }
    }

    fn size_helper(&self, node: &Option<Box<Node<T>>>) -> usize {
        match node {
            Some(node) => {
                1 + self.size_helper(&node.left) + self.size_helper(&node.right)
            },
            None => 0,
        }
    }
}


impl<T: Ord> Heap<T> for LeftistHeap<T> {
    fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.element)
    }

    fn pop(&mut self) -> Option<T> {
        self.root.take().map(|mut node| {
            let left = node.left.take();
            let right = node.right.take();
            self.root = Self::merge_nodes(left, right);
            node.element
        })
    }

    fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        self.root = Self::merge_nodes(self.root.take(), Some(node));
    }

    fn size(&self) -> usize {
        self.size_helper(&self.root)
    }

}


#[cfg(test)]
mod tests {
    use crate::ds::heap::Heap;

    use super::LeftistHeap;

    #[test]
    fn test_basic_heap() {
        let mut heap = LeftistHeap::<i32>::new();

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
        let mut heap: LeftistHeap<i32> = LeftistHeap { root: None };

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