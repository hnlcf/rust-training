use std::cell::UnsafeCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T>
where
    T: Debug + Copy,
{
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn test_cell() {
        let cell = Cell::new(1);

        println!("{:#?}", cell);
    }
}
