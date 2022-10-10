/// A trait for Binary Search Tree (e.g. AVL, splay, b-tree, rb-tree, ...).
pub trait BST<T> {
    type NodePtr;

    /// Find the target value and return the node.
    fn search(&self, value: &T) -> &Self::NodePtr;

    /// Insert the value and return the node inserted.
    fn insert(&mut self, value: T) -> &Self::NodePtr;

    /// Return `TRUE` iff delete the target node successfully.
    fn remove(&mut self, value: &T) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinTreeMap<K, V> {
    root: Option<Box<TreeNode<K, V>>>,
    size: usize,
}

impl<K, V> Default for BinTreeMap<K, V> {
    fn default() -> Self {
        BinTreeMap {
            root: None,
            size: 0,
        }
    }
}

impl<K, V> BST<V> for BinTreeMap<K, V> {
    type NodePtr = Option<Box<TreeNode<K, V>>>;

    fn search(&self, _value: &V) -> &Self::NodePtr {
        todo!()
    }

    fn insert(&mut self, value: V) -> &Self::NodePtr {
        let node = self.search(&value);
        if node.is_none() {
            todo!()
        } else {
            node
        }
    }

    fn remove(&mut self, value: &V) -> bool {
        let node = self.search(value);
        if node.is_some() {
            todo!()
        } else {
            false
        }
    }
}

impl<K, V> BinTreeMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct TreeNode<K, V> {
    rank: K,
    data: V,
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
}

impl<K, V> Default for TreeNode<K, V>
where
    K: Default,
    V: Default,
{
    fn default() -> Self {
        TreeNode {
            rank: K::default(),
            data: V::default(),
            left: None,
            right: None,
        }
    }
}

impl<K, V> Eq for TreeNode<K, V>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V> PartialEq<TreeNode<K, V>> for TreeNode<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &TreeNode<K, V>) -> bool {
        self.rank == other.rank
            && self.data == other.data
            && self.left == other.left
            && self.right == other.right
    }
}

impl<K, V> Ord for TreeNode<K, V>
where
    K: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl<K, V> PartialOrd for TreeNode<K, V>
where
    K: PartialOrd + Ord,
    V: PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bintree_default() {
        let t1 = BinTreeMap::<usize, String>::default();
        let t2 = BinTreeMap::default();

        assert_eq!(t1, t2);
    }
}
