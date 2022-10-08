#[allow(dead_code)]
#[derive(Debug)]
pub struct DisjointSet {
    data: Vec<Option<usize>>,
    len: usize,
}

#[allow(dead_code)]
impl DisjointSet {
    pub fn new(len: usize) -> Self {
        Self {
            data: vec![None; len],
            len,
        }
    }

    pub fn reset(&mut self, key: usize) {
        self.data[key] = None;
    }

    pub fn find(&self, key: usize) -> Option<usize> {
        let mut p = key;
        while let Some(val) = self.data[p] {
            p = val;
        }
        Some(p)
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let mut x = self.find(x).unwrap();
        let mut y = self.find(y).unwrap();
        if x == y {
            return;
        }

        if x > y {
            (x, y) = (y, x);
        }

        self.data[x] = Some(y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_set() {
        let mut pp = DisjointSet::new(10);

        pp.union(0, 9);
        pp.union(1, 0);
        pp.union(3, 1);
        pp.union(8, 6);
        pp.union(6, 3);

        println!("{:?}", pp);
    }
}
