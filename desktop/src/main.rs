mod app;
mod args;
mod keyboard;
mod sound;

use app::App;
use args::Args;
use clap::Parser;

fn main() {
    // Parse command line arguments.
    let args = Args::parse();

    // Initialize logging.
    pretty_env_logger::init();

    // Initialize application.
    let mut app = App::new(args);

    // Initialize event loop & run the application.
    app.run().unwrap();
}
