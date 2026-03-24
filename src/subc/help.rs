const HELP_BODY: &str = include_str!("../static/help.txt");

// *brakoll - d: overhaul readme, p: 20, t: docs, s: open

// app info
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERS: &str = env!("CARGO_PKG_VERSION");
const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
const APP_AUTH: &str = env!("CARGO_PKG_AUTHORS");

pub fn run() {
    println!("{APP_NAME} v{APP_VERS}");
    println!("{APP_DESC}");
    println!("{APP_REPO}");
    println!("{APP_AUTH}");
    println!("---");
    println!("{HELP_BODY}");
}
