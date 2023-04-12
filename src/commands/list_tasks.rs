use anyhow::Result;

use crate::{storage::Storage, task::Task};

pub fn list_tasks(storage: &Storage, completed_only: bool, pending_only: bool) -> Result<()> {
    let tasks = storage.load()?;

    if tasks.is_empty() {
        println!("No hay tareas. Usa 'rusty-todo add <descripción>' para agregar una.");
        return Ok(());
    }

    let filtered_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|t| {
            if completed_only {
                t.completed
            } else if pending_only {
                !t.completed
            } else {
                true
            }
        })
        .collect();

    if filtered_tasks.is_empty() {
        println!("No hay tareas que mostrar con los filtros aplicados.");
        return Ok(());
    }

    println!("\n📋 Tareas:\n");
    for task in &filtered_tasks {
        let status = if task.completed { "✓" } else { "○" };
        let created = task.created_at.format("%Y-%m-%d %H:%M");
        println!("  {} [{}] {}", status, task.id, task.description);
        println!("     Creada: {}", created);
        if let Some(completed_at) = task.completed_at {
            println!("     Completada: {}", completed_at.format("%Y-%m-%d %H:%M"));
        }
        println!();
    }
    println!("Total: {} tarea(s)", filtered_tasks.len());
    Ok(())
}
