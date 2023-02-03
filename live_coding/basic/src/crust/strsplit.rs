pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(content: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(content),
            delimiter,
        }
    }
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((start, end)) = self.delimiter.find_next(remainder) {
            let former = &remainder[..start];
            *remainder = &remainder[end..];

            Some(former)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .position(|(_, c)| c == *self)
            .map(|idx| (idx, idx + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("Strsplit always gives at least one result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_until_char() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }

    #[test]
    fn test_strsplit_general() {
        let string = "a b c d e";
        let pat = " ";

        let ss: Vec<_> = StrSplit::new(string, pat).collect();
        let target = vec!["a", "b", "c", "d", "e"];

        assert_eq!(ss, target);
    }

    #[test]
    fn test_strsplit_tail_space() {
        let string = "a b c d ";
        let pat = " ";

        let ss: Vec<_> = StrSplit::new(string, pat).collect();
        let target = vec!["a", "b", "c", "d", ""];

        assert_eq!(ss, target);
    }

    #[test]
    fn test_strsplit_no_pat() {
        let string = "abcde";
        let pat = " ";

        let ss: Vec<_> = StrSplit::new(string, pat).collect();
        let target = vec!["abcde"];

        assert_eq!(ss, target);
    }

    #[test]
    fn test_strsplit_empty() {
        let string = "";
        let pat = " ";

        let ss: Vec<_> = StrSplit::new(string, pat).collect();
        let target = vec![""];

        assert_eq!(ss, target);
    }
}
