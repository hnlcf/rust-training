#[allow(dead_code)]
fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(idx) => {
            let preffix = &s[..idx];
            let suffix = &s[idx + pat.len_utf8()..];
            *s = suffix;
            preffix
        }
        None => {
            let preffix = *s;
            *s = "";
            preffix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtok() {
        let mut s = "helloworld";
        let pat = ' ';
        assert_eq!(strtok(&mut s, pat), "helloworld");
        assert_eq!(s, "");
    }
}
