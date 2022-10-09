use std::collections::VecDeque;

use super::{
    edge::{Edge, EdgeState},
    vertex::{Vertex, VertexState},
};

pub trait Graph {
    fn connect_edge(&mut self, v1: usize, v2: usize);
    fn bfs(&mut self, v: usize);
    fn dfs(&mut self, v: usize);
    fn pfs(&mut self, v: usize);
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

impl<V, E> GraphMatrix<V, E> {
    pub fn size(&self) -> (usize, usize) {
        (self.n, self.e)
    }

    pub fn first_neighbor(&self, v: usize) -> Option<usize> {
        self.next_neighbor(v, self.n)
    }

    pub fn next_neighbor(&self, v: usize, previous: usize) -> Option<usize> {
        self.edges[v]
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if i < previous && e.is_some() {
                    Some(i)
                } else {
                    None
                }
            })
            .max()
    }

    pub fn is_exist_edge(&self, v1: usize, v2: usize) -> bool {
        self.edges[v1][v2].is_some()
    }

    pub fn get_vertex_ref(&self, v: usize) -> &Vertex<V> {
        self.vertices[v].as_ref()
    }

    pub fn get_vertex_mut(&mut self, v: usize) -> &mut Vertex<V> {
        self.vertices[v].as_mut()
    }

    pub fn get_vertex_state(&self, v: usize) -> &VertexState {
        self.get_vertex_ref(v).state()
    }

    pub fn update_vertex_state(&mut self, v: usize, state: VertexState) {
        self.get_vertex_mut(v).update_state(state)
    }

    pub fn update_vertex_parent(&mut self, v: usize, parent: usize) {
        self.get_vertex_mut(v).update_parent(parent)
    }

    pub fn get_edge_ref(&self, v1: usize, v2: usize) -> Option<&Edge<E>> {
        self.edges[v1][v2].as_ref()
    }

    pub fn get_edge_mut(&mut self, v1: usize, v2: usize) -> Option<&mut Edge<E>> {
        self.edges[v1][v2].as_mut()
    }

    pub fn get_edge_state(&self, v1: usize, v2: usize) -> Option<&EdgeState> {
        if let Some(e) = self.get_edge_ref(v1, v2) {
            Some(e.state())
        } else {
            None
        }
    }

    pub fn update_edge_state(&mut self, v1: usize, v2: usize, state: EdgeState) {
        if let Some(e) = self.get_edge_mut(v1, v2) {
            e.update_state(state)
        }
    }
}

impl<V, E> Graph for GraphMatrix<V, E>
where
    E: Default,
{
    fn connect_edge(&mut self, v1: usize, v2: usize) {
        self.edges[v1][v2] = Some(Edge::default());
    }

    fn bfs(&mut self, v: usize) {
        let mut queue = VecDeque::new();

        self.update_vertex_state(v, VertexState::Discovered);
        queue.push_back(v);

        while let Some(curr) = queue.pop_back() {
            let neighbor = self.n;
            while let Some(next) = self.next_neighbor(v, neighbor) {
                if self.get_vertex_state(next) == &VertexState::Undiscovered {
                    self.update_vertex_state(next, VertexState::Discovered);
                    self.update_edge_state(neighbor, next, EdgeState::Tree);
                    self.update_vertex_parent(next, neighbor);
                    queue.push_back(next);
                } else {
                    self.update_edge_state(neighbor, next, EdgeState::Cross);
                }
            }
            self.update_vertex_state(curr, VertexState::Discovered);
        }
    }

    fn dfs(&mut self, _v: usize) {
        todo!()
    }

    fn pfs(&mut self, _v: usize) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_new() {
        let g: GraphMatrix<u32, String> = GraphMatrix::default();
        assert_eq!(g.size(), (0, 0));
    }
}
