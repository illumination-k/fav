use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

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
    pub fn highlight<'a>(&self, extension: &str, content: &'a str) -> Result<Vec<Spans<'a>>> {
        let syntax = self
            .ps
            .find_syntax_by_extension(extension)
            .with_context(|| format!("Error: {}", extension))?;

        let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);

        Ok(content
            .split('\n')
            .map(|_line| {
                let mut spans = Vec::new();
                for line in LinesWithEndings::from(_line) {
                    spans.extend(h.highlight_line(line, &self.ps).unwrap().into_iter().map(
                        |(syntect_style, s)| {
                            let fg_color = Color::Rgb(
                                syntect_style.foreground.r,
                                syntect_style.foreground.g,
                                syntect_style.foreground.b,
                            );

                            let tui_style = Style::default().fg(fg_color);
                            Span::styled(s, tui_style)
                        },
                    ))
                }

                Spans::from(spans)
            })
            .collect())
    }
}
