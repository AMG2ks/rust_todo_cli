use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "To-Do CLI")]
#[command(version = "1.0")]
#[command(author = "Aziz Guebsi")]
#[command(about = "A simple to-do list app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { task: String },
    /// List all tasks
    List,
    /// Remove a task by its ID
    Remove { id: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let args = Cli::parse();
    let file_path = "tasks.json";
    let mut tasks = load_tasks(file_path);

    match args.command {
        Commands::Add { task } => {
            add_task(&task, &mut tasks);
            save_tasks(&tasks, file_path);
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Remove { id } => {
            remove_task(id, &mut tasks);
            save_tasks(&tasks, file_path);
        }
    }
}

fn load_tasks(file_path: &str) -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(file_path) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>, file_path: &str) {
    let data = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(file_path, data).expect("Failed to save tasks");
}

fn add_task(description: &str, tasks: &mut Vec<Task>) {
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        description: description.to_string(),
        completed: false,
    });
    println!("Added task: {}", description);
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("{} {}: {}", status, task.id, task.description);
    }
}

fn remove_task(id: usize, tasks: &mut Vec<Task>) {
    if let Some(pos) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(pos);
        println!("Removed task with ID: {}", id);
    } else {
        eprintln!("No task found with ID: {}", id);
    }
}
