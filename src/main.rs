mod commands;
mod storage;
mod task;

use anyhow::Result;
use clap::Parser;
use commands::{
    add_task, clean_completed, complete_task, delete_task, list_tasks, uncomplete_task, Commands,
};
use storage::Storage;

#[derive(Parser)]
#[command(name = "rusty-todo")]
#[command(about = "Un gestor de tareas CLI simple con persistencia JSON", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let storage = Storage::new()?;

    match cli.command {
        Commands::Add { description } => {
            add_task(&storage, description)?;
        }
        Commands::List { completed, pending } => {
            list_tasks(&storage, completed, pending)?;
        }
        Commands::Complete { id } => {
            complete_task(&storage, id)?;
        }
        Commands::Uncomplete { id } => {
            uncomplete_task(&storage, id)?;
        }
        Commands::Delete { id } => {
            delete_task(&storage, id)?;
        }
        Commands::Clean => {
            clean_completed(&storage)?;
        }
    }

    Ok(())
}
