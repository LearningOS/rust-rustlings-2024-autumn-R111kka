/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::clone::Clone;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Clone + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone+ Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        if self.count == 0 {
            self.items[0] = value.clone();
            self.count += 1;
            return;
        }
        let mut idx = self.count;
        self.count += 1;
        self.items.push(value);
        loop {
            let p_idx = self.parent_idx(idx);
            if idx == 0 || (self.comparator)(&self.items[p_idx], &self.items[idx]) {
                break;
            } else {
                self.items.swap(p_idx, idx);
                idx = p_idx;
            }
        }
        println!("items: {:?}", self.items);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, _idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone+ Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone+ Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            return None;
        }
        self.items.swap(0, self.count - 1);
        let result = self.items.pop();
        self.count -= 1;
        let mut idx = 0;
        while idx < self.count - 1 {
            let left_child_idx = self.left_child_idx(idx);
            let right_child_idx = self.right_child_idx(idx);
            if left_child_idx < self.count  && right_child_idx < self.count  {
                if (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
                    if (self.comparator)(&self.items[left_child_idx], &self.items[idx]) {
                        self.items.swap(left_child_idx, idx);
                        idx = left_child_idx;
                    } else {
                        break;
                    }
                } else {
                    if (self.comparator)(&self.items[right_child_idx], &self.items[idx]) {
                        self.items.swap(right_child_idx, idx);
                        idx = right_child_idx;
                    } else {
                        break;
                    }
                }
            } else if left_child_idx < self.count {
                if (self.comparator)(&self.items[left_child_idx], &self.items[idx]) {
                    self.items.swap(left_child_idx, idx);
                    idx = left_child_idx;
                } else {
                    break;
                } 
            } else if right_child_idx < self.count {
                if (self.comparator)(&self.items[right_child_idx], &self.items[idx]) {
                    self.items.swap(right_child_idx, idx);
                    idx = right_child_idx;
                } else {
                    break;
                } 
            } else {
                break;
            }
        }
        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}