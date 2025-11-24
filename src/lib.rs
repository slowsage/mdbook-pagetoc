use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use std::fs;
use tracing::info;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_env_var("RUST_LOG")
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(filter)
        .init();
}
pub mod pagetoc_lib {
    use super::*;

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
            init_logger();
            let html_config = ctx.config.html_config().unwrap_or_default();

            let pagetoc_js = include_str!("pagetoc.js");
            let pagetoc_css = include_str!("pagetoc.css");
            let theme_dir = match html_config.theme {
                Some(ref theme) => ctx.root.join(theme),
                None => ctx.root.join("theme"),
            };

            fs::create_dir_all(theme_dir.as_path()).expect("Unable to create directory");
            for (file_name, contents) in [("pagetoc.js", pagetoc_js), ("pagetoc.css", pagetoc_css)]
            {
                let file_path = theme_dir.join(file_name);
                if !file_path.exists() {
                    info!("{}: Writing {}", self.name(), file_path.display());
                    fs::write(file_path, contents).expect("Unable to write file");
                }
            }
            Ok(book)
        }

        fn supports_renderer(&self, renderer: &str) -> Result<bool> {
            Ok(renderer == "html")
        }
    }
}
