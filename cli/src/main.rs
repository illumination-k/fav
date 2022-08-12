use app::{run_app, App};
use clap::{Parser, Subcommand};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
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
            enable_raw_mode()?;

            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            let app = App::default();
            let res = run_app(&mut terminal, app);

            disable_raw_mode()?;

            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture,
            )?;
            terminal.show_cursor()?;

            if let Err(err) = res {
                println!("{:?}", err);
            }
        }
        Commands::New => {
            // get name and description
            // !TODO

            // get template by editor
            let editor = edit::Edit::new()
                .with_editor(Some("code -w".to_string()))
                .with_name(Some("test.py".to_string()));
            let edited = editor.edit("text")?;
            println!("{}", edited);
        }
        Commands::Export => {}
        Commands::Import => {}
    }
    Ok(())
}
