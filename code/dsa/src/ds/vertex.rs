#[derive(Debug)]
pub enum VertexStatus {
    Undiscovered,
    Discovered,
    Visited,
}

#[derive(Debug)]
pub struct Vertex<T> {
    data: T,
    status: VertexStatus,
    in_degree: usize,
    out_degree: usize,
    d_time: i32,
    f_time: i32,
    parent: Option<usize>,
    priority: i32,
}

impl<T> Vertex<T> {
    pub fn new(data: T) -> Self {
        Vertex {
            data,
            status: VertexStatus::Undiscovered,
            in_degree: 0,
            out_degree: 0,
            d_time: -1,
            f_time: -1,
            parent: None,
            priority: i32::MAX,
        }
    }
}
