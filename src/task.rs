use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    tasks: Vec<Task>
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self { tasks: Vec::new() } }
}

pub fn get_tasks() -> Result<Config, confy::ConfyError> {
    let  cfg: Config = confy::load("todo", "todo_list")?;
    Ok(cfg)
}
