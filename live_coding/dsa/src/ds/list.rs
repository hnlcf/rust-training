use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ListNode<T> {
    data: T,
    next: Option<RefCell<Box<ListNode<T>>>>,
}
