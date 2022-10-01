pub fn kmp(t: &str, p: &str) -> Option<usize> {
    let n = t.chars().count();
    let m = p.chars().count();
    let next = build_next(p);

    let mut j = 0;
    let mut i = 0;
    while j < m as isize && i < n {
        if j < 0 || t.chars().nth(i as usize) == p.chars().nth(j as usize) {
            i += 1;
            j += 1;
        } else {
            j = next[j as usize];
        }
    }

    Some(if j < 0 { i + 1 } else { i - j as usize })
}

fn build_next(s: &str) -> Vec<isize> {
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
        assert_eq!(build_next(s), vec![-1, -1, -1, -1, 3, -1]);
    }
}
