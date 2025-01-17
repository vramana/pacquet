use clap::{Parser, Subcommand};

/// Experimental package manager for node.js written in rust.
#[derive(Parser, Debug)]
#[command(name = "pacquet")]
#[command(bin_name = "pacquet")]
#[command(version = "0.0.1")]
#[command(about = "Experimental package manager for node.js")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Initialize a package.json
    Init,
    /// Add a package
    Add(AddArgs),
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    /// Name of the package
    pub package: String,
    /// The directory with links to the store (default is node_modules/.pacquet).
    /// All direct and indirect dependencies of the project are linked into this directory
    #[arg(long = "virtual-store-dir", default_value = "node_modules/.pacquet")]
    pub virtual_store_dir: String,
}
