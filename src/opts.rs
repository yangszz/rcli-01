use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "csv to json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(long, default_value = "true")]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err(format!("file not found: {}", file_name))
    }
}
