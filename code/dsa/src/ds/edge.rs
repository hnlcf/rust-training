#[allow(dead_code)]
#[derive(Debug)]
pub enum EdgeState {
    /// The edge in initial state
    Undetermined,
    /// The edge in the traversal tree
    Tree,
    /// The edge between two different branches
    Cross,
    /// The edge from child to ancestor
    Forward,
    /// The edge from ancestor to child
    Backward,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edge<T> {
    data: T,
    /// The weight of edge
    weight: u32,
    /// The status of edge
    status: EdgeState,
}

#[allow(dead_code)]
impl<T> Edge<T> {
    pub fn new(data: T, weight: u32) -> Self {
        Self {
            data,
            status: EdgeState::Undetermined,
            weight,
        }
    }
}
