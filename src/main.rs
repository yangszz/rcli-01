use clap::Parser;

use rcli::{csv_to_json, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    // rcli csv -i input.csv -o output.json --header -d ','
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            csv_to_json(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
