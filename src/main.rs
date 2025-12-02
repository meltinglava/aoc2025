use std::path::PathBuf;

use askama::Template;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Day of month. Defaults to today's day.
    day: Option<u8>,
}

#[derive(Template)]
#[template(path = "dayXX.rs.askama")]
struct DayXXTemplate {
    day: u8,
}

fn main() {
    let cli = Cli::parse();

    let day = match cli.day {
        Some(d) => d,
        None => jiff::Zoned::now().day() as u8,
    };
    let path = PathBuf::from("src").join(format!("day{:02}.rs", day));

    if !path.exists() {
        let template = DayXXTemplate { day };
        let rendered = template.render().unwrap();

        std::fs::write(&path, rendered).expect("Unable to write file");
        println!("Created file: {}", path.display());
    } else {
        println!("File already exists: {}", path.display());
    }
}
