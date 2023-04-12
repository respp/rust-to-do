use crate::storage::Storage;
use anyhow::Result;

pub fn delete_task(storage: &Storage, id: u32) -> Result<()> {
    let mut tasks = storage.load()?;
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() < initial_len {
        storage.save(&tasks)?;
        println!("✓ Tarea #{} eliminada", id);
    } else {
        anyhow::bail!("No se encontró una tarea con ID {}", id);
    }
    Ok(())
}
