use crate::task::Task;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let file_path = std::env::current_dir()?.join("rusty-todo.json");

        Ok(Self { file_path })
    }

    pub fn load(&self) -> Result<Vec<Task>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .with_context(|| format!("No se pudo leer el archivo: {:?}", self.file_path))?;

        let tasks: Vec<Task> =
            serde_json::from_str(&content).context("Error al parsear el JSON")?;

        Ok(tasks)
    }

    pub fn save(&self, tasks: &[Task]) -> Result<()> {
        let json =
            serde_json::to_string_pretty(tasks).context("Error al serializar las tareas a JSON")?;

        fs::write(&self.file_path, json)
            .with_context(|| format!("No se pudo escribir en el archivo: {:?}", self.file_path))?;

        Ok(())
    }
}
