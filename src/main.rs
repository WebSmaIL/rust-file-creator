use clap::Parser;
use std::fs::{create_dir, File};
use std::io::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Global name of the creating file tree
    #[arg(short, long)]
    name: String,

    /// Name of the template used
    #[arg(short, long)]
    template: String,

    /// Elements
    #[arg(short, long)]
    elements: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    println!("Name {}!", args.name);
    println!("Template {}!", args.template);
    println!("Elements {:?}!", args.elements);

    match args.elements {
        Some(elements) => println!("Elements length {:?}!", elements.len()),
        None => {}
    }
}

// rfc --source ./src/entities/page/Test --template fsd --name Entity --elements ui,model,styles,lib
// rfc add entities -n EntityName -e ui,model,styles,lib
//
// rfc_config.json
//
// templates/fsd.json
//
// {
//      ""
// }
