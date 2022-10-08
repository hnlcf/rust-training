#[allow(dead_code)]
#[derive(Debug)]
pub enum VertexState {
    /// The initial state
    Undiscovered,
    /// The vertex that has been discovered but not traverse its all neighbors
    Discovered,
    /// The vertex that has finished visit
    Visited,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Vertex<T> {
    data: T,
    /// The status of the vertex
    status: VertexState,
    /// The number of in edges
    in_degree: usize,
    /// The number of out edges
    out_degree: usize,
    /// The time of discovering the vertex
    start_time: i32,
    /// The time of finishing visit the vertex
    end_time: i32,
    /// The parent of the vertex in the traversal tree
    parent: Option<usize>,
    /// The priority of the vertex(less is higher)
    priority: u32,
}

#[allow(dead_code)]
impl<T> Vertex<T> {
    pub fn new(data: T) -> Self {
        Vertex {
            data,
            status: VertexState::Undiscovered,
            in_degree: 0,
            out_degree: 0,
            start_time: -1,
            end_time: -1,
            parent: None,
            priority: u32::MAX,
        }
    }
}
