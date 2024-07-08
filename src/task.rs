use serde_derive::{Serialize, Deserialize};

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

pub fn get_tasks() -> Result<Config, confy::ConfyError> {
    let cfg: Config = confy::load("todo", "todo_list")?;
    Ok(cfg)
}

pub fn set_tasks(cfg: &Config) -> Result<(), confy::ConfyError> {
    confy::store("todo", "todo_list", cfg)?;
    Ok(())
}
