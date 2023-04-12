use crate::Storage;
use anyhow::Result;

pub fn uncomplete_task(storage: &Storage, id: u32) -> Result<()> {
    let mut tasks = storage.load()?;
    let task = tasks.iter_mut().find(|t| t.id == id);

    match task {
        Some(t) => {
            t.uncomplete();
            storage.save(&tasks)?;
            println!("✓ Tarea #{} marcada como pendiente", id);
        }
        None => {
            anyhow::bail!("No se encontró una tarea con ID {}", id);
        }
    }
    Ok(())
}
