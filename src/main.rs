use clap::Parser;
use info::get_info_map;

mod error;
mod info;
mod pid;

#[derive(Parser)]
struct Cli {
    process_name: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let output = get_info_map(&args);

    match output {
        Ok(out) => {
            println!("{}", out);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
