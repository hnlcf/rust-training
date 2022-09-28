use std::cell::RefCell;

#[derive(Debug)]
pub struct ListNode<T> {
    data: T,
    next: Option<RefCell<Box<ListNode<T>>>>,
}
