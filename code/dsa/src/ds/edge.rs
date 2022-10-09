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
    state: EdgeState,
}

#[allow(dead_code)]
impl<T> Edge<T> {
    pub fn new(data: T, weight: u32) -> Self {
        Self {
            data,
            state: EdgeState::Undetermined,
            weight,
        }
    }

    pub fn state(&self) -> &EdgeState {
        &self.state
    }

    pub fn update_state(&mut self, state: EdgeState) {
        self.state = state;
    }
}

impl<T> Default for Edge<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default(), 1)
    }
}

impl<T> AsRef<Edge<T>> for Edge<T> {
    fn as_ref(&self) -> &Edge<T> {
        self
    }
}

impl<T> AsMut<Edge<T>> for Edge<T> {
    fn as_mut(&mut self) -> &mut Edge<T> {
        self
    }
}
