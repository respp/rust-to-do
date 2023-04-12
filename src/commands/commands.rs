use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add {
        description: String,
    },
    List {
        #[arg(short, long)]
        completed: bool,
        #[arg(short, long)]
        pending: bool,
    },
    Complete {
        id: u32,
    },
    Uncomplete {
        id: u32,
    },
    Delete {
        id: u32,
    },
    Clean,
}
