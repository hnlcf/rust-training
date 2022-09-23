use std::{collections::BTreeMap, fmt::Display};

const GRAPH_EXPAND_SCALE: f32 = 1.3;

pub struct Graph<T> {
    pub matrix: Vec<Vec<Option<usize>>>,
    pub nodes: BTreeMap<usize, Option<T>>,
}

impl<T: Ord> Graph<T> {
    pub fn new() -> Self {
        Self {
            matrix: vec![],
            nodes: BTreeMap::new(),
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        self.nodes.contains_key(&index)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn insert_none(&mut self, index: usize) {
        self.may_expand(index);
        self.nodes.insert(index, None);
    }

    pub fn insert_value(&mut self, index: usize, value: T) {
        self.may_expand(index);
        self.nodes.insert(index, Some(value));
    }

    pub fn connect_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.may_insert(from);
        self.may_insert(to);

        self.matrix[from][to] = Some(weight);
        self.matrix[to][from] = Some(weight);
    }

    pub fn max_key(&self) -> usize {
        match self.nodes.iter().max() {
            Some((&e, _)) => e,
            None => 0,
        }
    }

    fn may_expand(&mut self, index: usize) -> bool {
        if self.len() > index {
            false
        } else {
            let new_len = (index as f32 * GRAPH_EXPAND_SCALE) as usize + 2;
            while self.matrix.len() < new_len {
                self.matrix.push(vec![]);
            }
            for i in 0..new_len {
                while self.matrix[i].len() < new_len {
                    self.matrix[i].push(None);
                }
            }
            true
        }
    }

    fn may_insert(&mut self, index: usize) {
        if !self.nodes.contains_key(&index) {
            self.insert_none(index);

            println!("Graph: insert a new vertex ({index}, None)");
        }
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if self.remove_vertex(index) {
            for i in 0..self.max_key() {
                self.remove_edge(i, index);
            }
            true
        } else {
            false
        }
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        self.matrix[from][to] = None;
        self.matrix[to][from] = None;
    }

    fn remove_vertex(&mut self, index: usize) -> bool {
        if self.contains(index) {
            self.nodes.remove(&index);
            true
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        if self.is_empty() {
            format!("Graph: empty")
        } else {
            let max = self.max_key();
            let mut output = format!("  ");

            (0..=max).for_each(|x| output.push_str(&format!("{:<2}", x)));

            for i in 0..=max {
                output.push_str(&format!("\n{:<2}", i));

                for j in 0..=max {
                    output.push_str(&format!(
                        "{:2}",
                        (match self.matrix[i][j] {
                            Some(e) => format!("{}", e),
                            None => format!("."),
                        })
                    ))
                }
            }
            output
        }
    }
}

impl<T: Ord> Display for Graph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let g: Graph<String> = Graph::new();
        assert_eq!(g.len(), 0);
        assert!(g.is_empty());
    }
}
