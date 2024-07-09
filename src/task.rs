use serde_derive::{Serialize, Deserialize};
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub count: u32,
    pub completed_count: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tasks: Vec<Task>
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self { tasks: Vec::new() } }
}

pub fn get_config() -> Result<Config, confy::ConfyError> {
    let cfg: Config = confy::load("todo", "todo_list")?;
    Ok(cfg)
}

pub fn set_config(cfg: &Config) -> Result<(), confy::ConfyError> {
    confy::store("todo", "todo_list", cfg)?;
    Ok(())
}

pub fn display_tasks(tasks: &Vec<Task>) {
    println!("\nTasks to do:");
    for (index, task) in tasks.iter().enumerate() {
        let description = if task.count == task.completed_count {
            task.description.green()
        } else {
            task.description.normal()
        };
        let single = task.count == 1;

        print!("{index}. {description} ", index = index + 1, description = description);

        if !single {
            let status = format!(
                    "[{completed_count} / {count} Complete]",
                    completed_count = task.completed_count,
                    count = task.count
                );
            if task.count == task.completed_count {
                println!("{}", status.green());
            } else {
                println!("{status}");
            }
        } else {
            println!();
        } 
    }
    println!("");
}
