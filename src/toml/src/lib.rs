use std::{env, fmt::Display};

use syntect::{
    dumps::from_reader,
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::as_24_bit_terminal_escaped,
};

pub struct Highlighted {
    content: String,
}

impl Display for Highlighted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.content)
    }
}

#[derive(Debug)]
pub struct Highlighter {
    syntax_set: SyntaxSet,
    syntax: SyntaxReference,
    theme: Theme,
}

impl Highlighter {
    pub fn new() -> Self {
        let syntax_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/syntax.dump"));

        let syntax_set: SyntaxSet = from_reader(syntax_bytes as &[u8]).unwrap();
        let theme_set = ThemeSet::load_defaults();
        let syntax = syntax_set.find_syntax_by_extension("toml").unwrap().clone();
        let theme = theme_set.themes["base16-ocean.dark"].clone();

        Highlighter {
            syntax_set,
            syntax,
            theme,
        }
    }

    pub fn highlight(&self, source: &str) -> Highlighted {
        Highlighted {
            content: source
                .split('\n')
                .map(|line| {
                    as_24_bit_terminal_escaped(
                        &HighlightLines::new(&self.syntax, &self.theme)
                            .highlight(line, &self.syntax_set)[..],
                        false,
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
}
