use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

use crossterm::style::Color;

use anyhow::{Context, Result};
pub struct CodeHighligher {
    ps: SyntaxSet,
    ts: ThemeSet,
}

impl Default for CodeHighligher {
    fn default() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }
}

impl CodeHighligher {
    pub fn highlight(&self, extension: &str, content: &str) -> Result<Vec<(String, Color)>> {
        let syntax = self
            .ps
            .find_syntax_by_extension(extension)
            .with_context(|| format!("Error: {}", extension))?;

        let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);

        let mut content_with_colors = Vec::new();


        for line in LinesWithEndings::from(content) {
            content_with_colors.extend(h.highlight_line(line, &self.ps)?.into_iter().map(
                |(syntect_style, s)| {
                    let fg_color = Color::Rgb {
                        r: syntect_style.foreground.r,
                        g: syntect_style.foreground.g,
                        b: syntect_style.foreground.b,
                    };

                    (s.to_string(), fg_color)
                },
            ))
        }


        Ok(content_with_colors)
    }
}

