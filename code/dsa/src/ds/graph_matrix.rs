use super::{edge::Edge, vertex::Vertex};

#[derive(Debug)]
pub struct GraphMatrix<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Vec<Option<Edge<E>>>>,
    n: usize,
    e: usize,
}

impl<V, E> Default for GraphMatrix<V, E> {
    fn default() -> Self {
        Self {
            vertices: vec![],
            edges: vec![],
            n: 0,
            e: 0,
        }
    }
}
