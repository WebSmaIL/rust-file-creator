use clap::Parser;
use std::fs::{self, create_dir, File};
use std::io::prelude::*;

use serde::Serialize;
use serde_json;

mod commands;
use commands::commands_mod::Commands;

#[derive(Serialize)]
struct ConfigFile {
    default_template: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    // let args = Args::parse();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { template }) => {
            println!("Creating config directory...");
            let dir = fs::create_dir("rfc_config");
            match dir {
                Ok(()) => {
                    println!("Creating config file...");
                    let mut file = fs::File::create("./rfc_config/config.json").unwrap();
                    let config_content = ConfigFile {
                        default_template: String::from("testing"),
                    };
                    let json = serde_json::to_string(&config_content).unwrap();
                    file.write_all(json.as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }
        }
        None => {
            println!("Unknown command!!!");
        }
    }

    // println!("Name {}!", args.name);
    // println!("Template {}!", args.template);
    // println!("Elements {:?}!", args.elements);

    // match args.elements {
    //     Some(elements) => println!("Elements length {:?}!", elements.len()),
    //     None => {}
    // }
}

// rfc --source ./src/entities/page/Test --template fsd --name Entity --elements ui,model,styles,lib
// rfc add entities -n EntityName -e ui,model,styles,lib
//
// rfc_config.json
//ww.xvideos.com (unsafe)
// templates/fsd.json
//
// {
//      ""
// }
