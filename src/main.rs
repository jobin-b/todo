use clap::{Parser, Subcommand};
mod task;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds tasks to todo list
    Add { 
        /// Task description
        task: String,
        /// Optional task count
        #[arg(short,long,default_value_t=1)]
        count: u32,
        /// Time to notify user at
        // TODO: do this later maybe
        #[cfg(target_os="linux")]
        notify: Option<String>,
        
    },
    /// View all tasks
    View,
    Done {
        /// ID of the task
        task_id:u32,
        /// Number of times done (ignored if task has count of 1)
        count:u32,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { task, count, .. } => {
            let mut config = task::get_config()?;
            let new_task = task::Task {
                description: task.clone(),
                count: count.clone(),
                completed_count: 0,
            };
            config.tasks.push(new_task);
            task::set_config(&config)?;
            println!("Successfully Added to list!");
            task::display_tasks(&config.tasks);
        }
        Commands::View => {
            let tasks = task::get_config()?.tasks;
            task::display_tasks(&tasks)
            // Implement view logic here
        }
        Commands::Done { task_id, count } => {
            println!("Completing task ID: {}", task_id);
            println!("Count: {}", count);
            // Implement completion logic here
        }
    }
    Ok(())
}
