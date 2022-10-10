#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(content: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(content),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(pos) = remainder.find(self.delimiter) {
            let former = &remainder[..pos];
            *remainder = &remainder[(pos + self.delimiter.len())..];

            Some(former)
        } else {
            self.remainder.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StrSplit;

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
