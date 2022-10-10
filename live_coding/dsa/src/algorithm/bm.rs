use std::collections::HashMap;

pub fn bm(s: &str, pat: &str) -> Option<usize> {
    let s_char = s.chars().collect::<Vec<_>>();
    let pat_char = pat.chars().collect::<Vec<_>>();
    let n = s_char.len();
    let m = pat_char.len();

    if m > n {
        return None;
    }

    if m == 0 {
        return Some(0);
    }

    let bc = build_bc(pat);
    let _gs = build_gs(pat);

    let mut i = 0;
    let mut j = m - 1;
    while i <= n - m {
        if s_char[i + j] == pat_char[j] {
            if j == 0 {
                break;
            }
            j -= 1;
        } else {
            match bc.get(&s_char[i + j]) {
                Some(&t) => {
                    if t > j {
                        i += 1;
                    } else {
                        i += j - t;
                    }
                }
                None => i += j + 1,
            }

            j = m - 1;
        }
    }

    if i > n - m {
        None
    } else {
        Some(i)
    }
}

/// Create bad character table
fn build_bc(pat: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for (i, c) in pat.chars().enumerate() {
        map.insert(c, i);
    }
    map
}

/// Create goof suffix table
fn build_gs(pat: &str) -> Vec<Option<usize>> {
    let n = pat.chars().count();
    vec![None; n]
}

#[allow(dead_code)]
fn build_ss(_pat: &str) -> Vec<Option<usize>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bm() {
        let s1 = "CHINCHILLA";
        let s2 = "CHIL";
        assert_eq!(bm(s1, s2), s1.find(s2));

        let s1 = "CHINCHILLA";
        let s2 = "CHII";
        assert_eq!(bm(s1, s2), s1.find(s2));

        let s1 = "";
        let s2 = "";
        assert_eq!(bm(s1, s2), s1.find(s2));

        let s1 = "CHINCHILLA";
        let s2 = "";
        assert_eq!(bm(s1, s2), s1.find(s2));

        let s1 = "";
        let s2 = "CHIL";
        assert_eq!(bm(s1, s2), s1.find(s2));
    }
}
