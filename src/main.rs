//! Browser app package: the application itself is the web UI in `app/`.
//! This small command exists so the package has a valid runtime section and a
//! version check; it is not what the App Center window runs.

const VERSION: &str = "1.0.4";

fn main() {
    if std::env::args().any(|arg| arg == "--version" || arg == "-V") {
        println!("Browser {VERSION}");
        return;
    }
    println!("Browser {VERSION} — open this app from RAST App Center.");
}
