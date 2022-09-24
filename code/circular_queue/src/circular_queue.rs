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
    /// The tail index plus 1, and the head index iff queue is full
    idx: usize,
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
            idx: 0,
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
        if self.capacity() == 0 {
            return false;
        }

        if !self.is_full() {
            self.data.push(val);
        } else {
            *self.data.get_mut(self.idx).unwrap() = val;
        }

        self.idx = self.inc_idx();

        true
    }

    #[inline]
    fn inc_idx(&self) -> usize {
        (self.idx + 1) % self.capacity()
    }

    #[inline]
    fn dec_idx(&self) -> usize {
        (self.idx + self.capacity() - 1) % self.capacity()
    }

    #[inline]
    pub fn front(&self) -> Option<T> {
        if !self.is_full() {
            self.data.first().copied()
        } else {
            self.data.get(self.idx).copied()
        }
    }

    #[inline]
    pub fn back(&self) -> Option<T> {
        self.data.get(self.dec_idx()).copied()
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let (a, b) = self.data.split_at(self.idx);
        a.iter().rev().chain(b.iter().rev())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a, b) = self.data.split_at_mut(self.idx);
        a.iter_mut().rev().chain(b.iter_mut().rev())
    }

    #[inline]
    pub fn asc_iter(&self) -> AscIter<T> {
        let (a, b) = self.data.split_at(self.idx);
        b.iter().chain(a.iter())
    }

    #[inline]
    pub fn asc_iter_mut(&mut self) -> AscIterMut<T> {
        let (a, b) = self.data.split_at_mut(self.idx);
        b.iter_mut().chain(a.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_queue() {
        let mut queue = CircularQueue::new(3);
        assert!(!queue.is_full());
        assert!(queue.is_empty());

        queue.push(1);
        queue.push(2);
        assert!(!queue.is_full());
        assert!(!queue.is_empty());
        assert_eq!(Some(1), queue.front());
        assert_eq!(Some(2), queue.back());

        queue.push(3);
        assert!(queue.is_full());
        assert!(!queue.is_empty());
        assert_eq!(Some(1), queue.front());
        assert_eq!(Some(3), queue.back());

        queue.push(4);
        assert_eq!(Some(2), queue.front());
        assert_eq!(Some(4), queue.back());

        queue.push(5);
        assert_eq!(Some(3), queue.front());
        assert_eq!(Some(5), queue.back());

        queue.push(6);
        assert_eq!(Some(4), queue.front());
        assert_eq!(Some(6), queue.back());

        queue.push(7);
        assert_eq!(Some(5), queue.front());
        assert_eq!(Some(7), queue.back());

        println!("{:?}", queue);

        for e in queue.iter() {
            println!("{:?}", e);
        }
    }
}
