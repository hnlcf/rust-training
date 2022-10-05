#[derive(Debug)]
pub enum EdgeStatus {
    Undetermined,
    Tree,
    Cross,
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct Edge<T> {
    data: T,
    status: EdgeStatus,
    weight: u32,
}

impl<T> Edge<T> {
    pub fn new(data: T, weight: u32) -> Self {
        Self {
            data,
            status: EdgeStatus::Undetermined,
            weight,
        }
    }
}
