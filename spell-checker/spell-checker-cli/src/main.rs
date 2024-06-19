use std::{fs, path::PathBuf, process::Command};

use anyhow::{Error, Result};
use clap::Parser;
use spell_checker::SpellChecker;
use std::process::Stdio;

///////////////////////////////////////////////////////////////////////////////

mod cli;

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let args = cli::Config::parse();

    // get the dictionary file path from the arguments or find a default
    let source = args.path.unwrap_or_else(|| find_default().unwrap());

    // create spell checker
    let mut spell_checker = SpellChecker::from_file(&source)?;

    match args.command {
        cli::Command::Check { text } => {
            for word in text {
                if !spell_checker.check(&word) {
                    println!("{}", word);
                }
            }
        }
        cli::Command::Suggest { word, limit } => {
            if !spell_checker.check(&word) {
                for suggestion in spell_checker.suggest(&word).iter().take(limit) {
                    println!("{}", suggestion);
                }
                println!();
            }
        }
        cli::Command::CheckSuggest { text, limit } => {
            for word in text {
                if !spell_checker.check(&word) {
                    println!("~~~ {}", word);
                    for suggestion in spell_checker.suggest(&word).iter().take(limit) {
                        println!("    {}", suggestion);
                    }
                    println!();
                }
            }
        }
        cli::Command::Add { word } => {
            if !spell_checker.check(&word) {
                spell_checker.add_word(&word);
                fs::write("/tmp/tmp.dic", spell_checker.to_string())?;
            }
        }
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn find_default() -> Result<PathBuf> {
    println!("No dictionary provided, searching default paths...");
    if cfg!(target_os = "linux") {
        let output = Command::new("fd")
            .arg("-e")
            .arg("dic")
            .current_dir("/")
            .stdout(Stdio::piped())
            .output()
            .expect("Process failed to execute");

        let out = String::from_utf8(output.stdout)?;

        let paths: Vec<&str> = out.split('\n').collect();

        println!("Found {} possible sources", paths.len());
        println!();

        for path in paths {
            if path.contains("en_US")
                || path.contains("en_GB")
                || path.contains("standard")
                || path.contains("default")
            {
                let path = "/".to_owned() + path;

                println!("Checking {}", path);

                match fs::read_to_string(path.clone()) {
                    Ok(contents) => {
                        if contents.contains("probably") {
                            println!("Using {}", path.clone());

                            println!();
                            println!("{:-^80}", "");
                            println!();

                            return Ok(PathBuf::from(path));
                        }
                    }
                    Err(err) => println!("Failed to open {} because {}", path, err),
                };
            }
        }
    }
    Err(Error::msg("Unable to find default"))
}

///////////////////////////////////////////////////////////////////////////////
