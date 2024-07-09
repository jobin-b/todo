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
    #[command(aliases = &["a"])]
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
    /// Complete/Progress a task
    Done {
        /// ID of the task
        task_id: u32,
        /// Number of times done (optional)
        #[arg(short,long,default_value_t=1)]
        count: u32,
    },
    #[command(aliases = &["rm"])]
    /// remove task by id
    Remove {
       /// ID of the task
       task_id: u32,
    },
    /// Clear the entire list
    Clear,
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
        }
        Commands::Done { task_id, count } => {
            println!("Completing task ID: {}", task_id);
            println!("Count: {}", count);
            let index = *task_id as usize;
            let mut config = task::get_config()?;
            let tasks = &mut config.tasks;
            assert!(index <= tasks.len(), "Task {index:?} does not exist");
            let task = &mut tasks[index-1];
            assert!(count + task.completed_count <= task.count, "Completed count exceeds task's limit");
            task.completed_count += count;
            task::set_config(&config)?;
            task::display_tasks(&config.tasks);
        }
        Commands::Remove { task_id } => {
            let mut config = task::get_config()?;
            let tasks = &mut config.tasks;
            let index = *task_id as usize;
            assert!(index <= tasks.len(), "Task {index:?} does not exist");
            tasks.remove(index - 1);
            task::set_config(&config)?;
            task::display_tasks(&config.tasks);
        }
        Commands::Clear => {
            let cfg = task::Config::default();
            task::set_config(&cfg)?;
            println!("Cleared todo list");
        }
    }
    Ok(())
}
