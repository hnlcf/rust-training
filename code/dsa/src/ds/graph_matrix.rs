use super::{edge::Edge, vertex::Vertex};

pub trait Graph {
    fn connect_edge(&mut self, v1: usize, v2: usize);
}

#[allow(dead_code)]
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
