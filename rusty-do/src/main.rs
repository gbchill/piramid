use clap::{Parser, Subcommand};

mod models;
use models::{TodoList, Priority};

mod tui; 

#[derive(Parser)]
#[command(author, version, about = "A Rusty Todo List CLI & TUI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        #[arg(short, long)]
        priority: Option<String>,
        #[arg(short, long)]
        deadline: Option<String>,
    },
    List {
        #[arg(short, long)]
        sort: Option<String>,
    },
    Done { id: u64 },
    Delete { id: u64 },
    Purge,
    Interactive, 
}

fn main() {
    let args = Cli::parse();
    let mut todolist = TodoList::new();

    match &args.command {
        Some(Commands::Add { name, priority, deadline }) => {
            if todolist.add_item(name.clone(), priority.clone(), deadline.clone()) {
                todolist.save().expect("Failed to save");
            }
        },
        Some(Commands::List { sort }) => {
            if let Some(s) = sort {
                match s.to_lowercase().as_str() {
                    "priority" => todolist.sort_by_priority(),
                    "deadline" => todolist.sort_by_deadline(),
                    _ => println!("Unknown sort type."),
                }
            }
            todolist.print();
        },
        Some(Commands::Done { id }) => {
            if todolist.complete_item(*id) {
                todolist.save().expect("Failed to save");
            } else { println!("Task {} not found", id); }
        },
        Some(Commands::Delete { id }) => {
            if todolist.delete_item(*id) {
                todolist.save().expect("Failed to save");
                println!("Deleted task {}", id);
            } else { println!("Task {} not found", id); }
        },
        Some(Commands::Purge) => {
            todolist.delete_all_completed();
            todolist.save().expect("Failed to save");
        },
        Some(Commands::Interactive) | None => {
            if let Err(e) = tui::run_tui(todolist) {
                eprintln!("TUI Error: {}", e);
            }
        }
    }
}
