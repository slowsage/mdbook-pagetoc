use clap::{Arg, ArgMatches, Command};
use mdbook_pagetoc::PagetocPreprocessor;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, parse_input};
use semver::{Version, VersionReq};
use std::io;
use std::process;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_env_var("RUST_LOG")
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .without_time()
        .with_writer(std::io::stderr)
        .with_env_filter(filter)
        .init();
}

pub fn make_app() -> Command {
    Command::new("mdbook-pagetoc")
        .about("A mdbook preprocessor which adds a TOC for the page")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    init_logger();
    let matches = make_app().get_matches();
    let preprocessor = PagetocPreprocessor::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    let (ctx, book) = parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook_preprocessor::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer).unwrap();
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
