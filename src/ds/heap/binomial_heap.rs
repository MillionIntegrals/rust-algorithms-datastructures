use super::Heap;

#[derive(Clone)]
pub struct Node<T: Ord> {
    element: T,
    children: Vec<Node<T>>,
}

#[derive(Clone)]
pub struct BinomialHeap<T: Ord + Clone> {
    trees: Vec<Node<T>>,
}

impl<T: Ord + Clone> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }

    fn merge(node1: Node<T>, node2: Node<T>) -> Node<T> {
        if node1.element < node2.element {
            let mut node = node1.clone();
            node.add_child(node2);
            node
        } else {
            let mut node = node2.clone();
            node.add_child(node1);
            node
        }
    }
}

impl<T: Ord + Clone> BinomialHeap<T> {
    pub fn new() -> Self {
        BinomialHeap { trees: Vec::new() }
    }

    pub fn merge_heap(&mut self, other: BinomialHeap<T>) {
        self.trees.append(&mut other.trees.clone());
        self.trees
            .sort_by(|a, b| a.children.len().cmp(&b.children.len()));
        let mut i = 0;
        while i + 1 < self.trees.len() {
            if self.trees[i].children.len() == self.trees[i + 1].children.len() {
                let node = Node::merge(self.trees.remove(i), self.trees.remove(i));
                self.trees.insert(i, node);
            } else {
                i += 1;
            }
        }
    }
}

impl<T: Ord + Clone> Heap<T> for BinomialHeap<T> {
    fn push(&mut self, element: T) {
        let node = Node::new(element);
        self.merge_heap(BinomialHeap { trees: vec![node] });
    }

    fn peek(&self) -> Option<&T> {
        self.trees
            .iter()
            .min_by(|x, y| x.element.cmp(&y.element))
            .map(|node| &node.element)
    }

    fn pop(&mut self) -> Option<T> {
        if self.trees.is_empty() {
            return None;
        }

        // Find the tree with the minimum root
        let min_index = self
            .trees
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.element.cmp(&y.element))
            .map(|(index, _)| index)?;

        // Remove the minimum root
        let Node {
            element,
            mut children,
        } = self.trees.remove(min_index);

        // Make each child a new tree
        children.reverse();
        let other = BinomialHeap { trees: children };

        // Merge the original heap with the new heap
        self.merge_heap(other);

        Some(element)
    }

    fn size(&self) -> usize {
        self.trees
            .iter()
            .map(|t| 2_usize.pow(t.children.len() as u32))
            .sum()
    }

    fn clear(&mut self) {
        self.trees.clear()
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::heap::Heap;

    use super::BinomialHeap;

    #[test]
    fn test_basic_heap() {
        let mut heap = BinomialHeap::<i32>::new();

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
        let mut heap: BinomialHeap<i32> = BinomialHeap::new();

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
