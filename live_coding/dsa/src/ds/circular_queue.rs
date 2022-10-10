use std::iter::{Chain, Rev};
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};

/// An iterator over `CircularQueue<T>`.
pub type Iter<'a, T> = Chain<Rev<SliceIter<'a, T>>, Rev<SliceIter<'a, T>>>;

/// A mutable iterator over `CircularQueue<T>`.
pub type IterMut<'a, T> = Chain<Rev<SliceIterMut<'a, T>>, Rev<SliceIterMut<'a, T>>>;

/// An ascending iterator over `CircularQueue<T>`.
pub type AscIter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;

/// An mutable ascending iterator over `CircularQueue<T>`.
pub type AscIterMut<'a, T> = Chain<SliceIterMut<'a, T>, SliceIterMut<'a, T>>;

#[derive(Debug, Clone)]
pub struct CircularQueue<T> {
    data: Vec<T>,
    capacity: usize,
    head: usize,
    tail: usize,
}

impl<T> CircularQueue<T>
where
    T: Copy + std::fmt::Debug,
{
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
            head: 0,
            tail: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    #[inline]
    pub fn push(&mut self, val: T) -> bool {
        if self.capacity() == 0 || self.is_full() {
            return false;
        }

        self.data.insert(self.tail, val);
        self.tail = (self.tail + 1) % self.capacity();

        true
    }

    #[inline]
    pub fn pop(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.data.remove(self.head);
        self.head = (self.head + 1) % self.capacity();
        true
    }

    #[inline]
    pub fn front(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.data.get(self.head).copied()
    }

    #[inline]
    pub fn back(&self) -> Option<T> {
        self.data
            .get((self.tail + self.capacity() - 1) % self.capacity())
            .copied()
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let (a, b) = self.data.split_at(self.head);
        a.iter().rev().chain(b.iter().rev())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a, b) = self.data.split_at_mut(self.head);
        a.iter_mut().rev().chain(b.iter_mut().rev())
    }

    #[inline]
    pub fn asc_iter(&self) -> AscIter<T> {
        let (a, b) = self.data.split_at(self.head);
        b.iter().chain(a.iter())
    }

    #[inline]
    pub fn asc_iter_mut(&mut self) -> AscIterMut<T> {
        let (a, b) = self.data.split_at_mut(self.head);
        b.iter_mut().chain(a.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_queue() {
        let mut queue = CircularQueue::new(3);
        assert!(queue.push(1));
        assert!(queue.push(2));
        assert!(queue.push(3));
        assert_eq!(queue.len(), 3);

        assert!(queue.is_full());
        assert!(!queue.push(4));

        assert_eq!(queue.back(), Some(3));
        assert!(queue.is_full());

        assert!(queue.pop());
        assert!(queue.push(4));

        assert_eq!(queue.back(), Some(4));
    }
}
