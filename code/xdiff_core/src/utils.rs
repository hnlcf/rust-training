use anyhow::Result;
use std::fmt::{self, Write};

use console::{style, Style};
use similar::{ChangeTag, TextDiff};

use syntect::{
    easy::HighlightLines,
    highlighting::{self, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn diff_text(text1: &str, text2: &str) -> Result<String> {
    let mut output = String::new();
    let diff = TextDiff::from_lines(text1, text2);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            writeln!(&mut output, "{:-^1$}", "-", 80)?;
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                write!(
                    &mut output,
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                )?;
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        write!(&mut output, "{}", s.apply_to(value).underlined().on_black())?;
                    } else {
                        write!(&mut output, "{}", s.apply_to(value))?;
                    }
                }
                if change.missing_newline() {
                    writeln!(&mut output)?;
                }
            }
        }
    }
    Ok(output)
}

pub fn highlight_text(text: &str, extension: &str) -> Result<String> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set.find_syntax_by_extension(extension).unwrap();

    let mut output = String::new();
    let mut highlight = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(text) {
        let ranges: Vec<(highlighting::Style, &str)> =
            highlight.highlight_line(line, &syntax_set)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);

        write!(&mut output, "{}", escaped)?;
    }

    Ok(output)
}
