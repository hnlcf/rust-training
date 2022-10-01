pub struct KmpPattern {
    pat: Vec<char>,
    next: Vec<Option<usize>>,
    len: usize,
}

impl KmpPattern {
    pub fn new(pat: &str) -> Self {
        let mut obj = Self {
            pat: pat.chars().collect(),
            next: vec![],
            len: pat.chars().count(),
        };
        obj.build_next();
        obj
    }

    pub fn kmp_find_in(&self, s: &str) -> Option<usize> {
        let s: Vec<_> = s.chars().collect();
        let n: usize = s.len();
        if self.len > n || self.len == 0 {
            return None;
        }

        let mut i: usize = 0;
        let mut j: Option<usize> = None;
        while j.unwrap_or(0) < self.len && i < n {
            if j.is_none() || s[i as usize] == self.pat[j.unwrap()] {
                i += 1;

                if let Some(idx) = j {
                    j = Some(idx + 1);
                } else {
                    j = Some(0);
                }
            } else {
                j = self.next[j.unwrap()];
            }
        }

        Some(if let Some(idx) = j { i - idx } else { i + 1 })
    }

    fn build_next(&mut self) {
        let mut next = vec![None];
        let mut t = next[0];
        let mut i = 0;
        while i < self.len - 1 {
            if t.is_none() || self.pat[i] == self.pat[t.unwrap()] {
                i += 1;

                if let Some(idx) = t {
                    t = Some(idx + 1);
                } else {
                    t = Some(0);
                }

                if self.pat[i] != self.pat[t.unwrap()] {
                    next.insert(i, t);
                } else {
                    next.insert(i, next[t.unwrap()]);
                }
            } else {
                t = next[t.unwrap()];
            }
        }
        self.next = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_pattern() {
        let s1 = "0001000010";
        let s2 = "000010";

        let p = KmpPattern::new(s2);
        assert_eq!(p.kmp_find_in(s1), s1.find(s2));

        let s1 = "CHINALLCHILLA";
        let s2 = "CHIL";

        let p = KmpPattern::new(s2);
        assert_eq!(p.kmp_find_in(s1), s1.find(s2));
    }
}

mod raw {
    fn kmp(s: &str, pat: &str) -> Option<usize> {
        let n = s.chars().count();
        let m = pat.chars().count();
        if m > n || m == 0 {
            return None;
        }

        let next = kmp_build_next(pat);

        let mut j = 0;
        let mut i = 0;
        while j < m as isize && i < n {
            if j < 0 || s.chars().nth(i as usize) == pat.chars().nth(j as usize) {
                i += 1;
                j += 1;
            } else {
                j = next[j as usize];
            }
        }

        Some(if j < 0 { i + 1 } else { i - j as usize })
    }

    fn kmp_build_next(s: &str) -> Vec<isize> {
        let n = s.chars().count();
        let mut next = vec![-1];
        let mut t = next[0];
        let mut i = 0;
        while i < n - 1 {
            if t < 0 || s.chars().nth(i) == s.chars().nth(t as usize) {
                i += 1;
                t += 1;

                if s.chars().nth(i) != s.chars().nth(t as usize) {
                    next.insert(i, t);
                } else {
                    next.insert(i, next[t as usize]);
                }
            } else {
                t = next[t as usize];
            }
        }
        next
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_kmp() {
            let s1 = "0001000010";
            let s2 = "000010";
            assert_eq!(kmp(s1, s2), s1.find(s2));

            let s1 = "CHINALLCHILLA";
            let s2 = "CHIL";
            assert_eq!(kmp(s1, s2), s1.find(s2));
        }

        #[test]
        fn test_build_next() {
            let s = "000010";
            assert_eq!(kmp_build_next(s), vec![-1, -1, -1, -1, 3, -1]);
        }
    }
}
