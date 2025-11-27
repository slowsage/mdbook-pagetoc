use mdbook_core::utils::fs;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use serde::Deserialize;
use std::path::Path;
use tracing::info;

#[derive(Debug, Deserialize)]
#[serde(default)]
struct PagetocConfig {
    scroll_offset: i32,
}

impl Default for PagetocConfig {
    fn default() -> Self {
        Self { scroll_offset: 10 }
    }
}

pub struct PagetocPreprocessor;

impl Default for PagetocPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl PagetocPreprocessor {
    pub fn new() -> PagetocPreprocessor {
        PagetocPreprocessor
    }
}

impl Preprocessor for PagetocPreprocessor {
    fn name(&self) -> &str {
        "mdbook-pagetoc"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let html_config = ctx.config.html_config().unwrap_or_default();

        let config: PagetocConfig = ctx
            .config
            .get("preprocessor.pagetoc")
            .ok()
            .flatten()
            .unwrap_or_default();

        let pagetoc_js = include_str!("pagetoc.js")
            .replace("{{SCROLL_OFFSET}}", &config.scroll_offset.to_string());
        let pagetoc_css = include_str!("pagetoc.css");

        let theme_dir = ctx
            .root
            .join(html_config.theme.as_deref().unwrap_or(Path::new("theme")));

        for (name, contents) in [
            ("pagetoc.js", pagetoc_js.as_str()),
            ("pagetoc.css", pagetoc_css),
        ] {
            let path = theme_dir.join(name);
            if !path.exists() {
                info!("Writing {}", path.display());
                fs::write(&path, contents)?;
            }
        }
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}
