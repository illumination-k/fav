use crate::templates::{ITemaplteRepository, JSONTemplateRepository, Template};
use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{execute, style::{SetForegroundColor, Print, ResetColor}};
use std::{
    error::Error,
    io::{BufRead, stdout},
};

pub mod edit;
pub mod highlighter;
pub mod templates;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List,
    New,
    Export,
    Import,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::List => {
        }
        Commands::New => {
            // get name and description
            // !TODO
            let p = "tmp/test.json".to_string();
            let mut repo = JSONTemplateRepository::load(&p)?;

            fn get_name_and_description_from_stdin() -> Result<(String, Option<String>)> {
                let mut name = String::new();
                let stdin = std::io::stdin();
                println!("Set template name with extension");
                {
                    let mut handle = stdin.lock();
                    handle.read_line(&mut name)?;
                }

                let mut description = String::new();
                println!("Set template description (Optional)");
                {
                    let mut handle = stdin.lock();
                    handle.read_line(&mut description)?;
                }

                let description = description.trim().to_string();

                if description.is_empty() {
                    Ok((name.trim().to_string(), None))
                } else {
                    Ok((name.trim().to_string(), Some(description)))
                }
            }

            let (name, description) = get_name_and_description_from_stdin()?;

            // get template by editor
            let editor = edit::Edit::new()
                .with_editor(Some("vim".to_string()))
                .with_name(Some(name.clone()));
            let edited = editor.edit("text")?;
            let mut template = Template::new(name, description, edited);
            template.highlight()?;

            for (line, color) in template.highlighted_template.as_ref().unwrap().iter() {
                execute!(stdout(), SetForegroundColor(*color), Print(line), ResetColor)?;
            }
            
            repo.put(template)?;

            repo.save(&p)?;
        }
        Commands::Export => {}
        Commands::Import => {}
    }
    Ok(())
}
