use crate::storage::Storage;
use anyhow::Result;

pub fn clean_completed(storage: &Storage) -> Result<()> {
    let mut tasks = storage.load()?;
    let initial_len = tasks.len();
    tasks.retain(|t| !t.completed);
    let removed = initial_len - tasks.len();

    if removed > 0 {
        storage.save(&tasks)?;

        println!("✓ {} tarea(s) completada(s) eliminada(s)", removed);
    } else {
        println!("No hay tareas completadas para eliminar");
    }
    Ok(())
}
