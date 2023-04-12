use crate::{storage::Storage, task::Task};
use anyhow::Result;

pub fn add_task(storage: &Storage, description: String) -> Result<()> {
    let mut tasks = storage.load()?;
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let task = Task::new(new_id, description);
    tasks.push(task);
    storage.save(&tasks)?;
    println!("tqarea agregada exitosamente");
    Ok(())
}
