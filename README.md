# Rusty-Todo-CLI

Un gestor de tareas simple y eficiente para la línea de comandos, escrito en Rust.

## Características

- ✅ Agregar, listar, completar y eliminar tareas
- 💾 Persistencia automática en JSON (archivo: `~/.rusty-todo.json`)
- 📅 Timestamps de creación y completado
- 🔍 Filtrar tareas por estado (completadas/pendientes)
- 🧹 Limpiar todas las tareas completadas de una vez

## Instalación

### Desde el código fuente

```bash
git clone <repo-url>
cd rusty-todo
cargo build --release
```

El binario estará en `target/release/rusty-todo-cli`. Puedes agregarlo a tu PATH o crear un alias:

```bash
alias todo='~/path/to/rusty-todo/target/release/rusty-todo-cli'
```

### Con Cargo install (si está publicado)

```bash
cargo install rusty-todo-cli
```

## Uso

### Agregar una tarea

```bash
rusty-todo add "Comprar leche"
rusty-todo add "Llamar al dentista"
```

### Listar todas las tareas

```bash
rusty-todo list
```

### Listar solo tareas completadas

```bash
rusty-todo list --completed
```

### Listar solo tareas pendientes

```bash
rusty-todo list --pending
```

### Completar una tarea

```bash
rusty-todo complete 1
```

### Marcar una tarea como pendiente

```bash
rusty-todo uncomplete 1
```

### Eliminar una tarea

```bash
rusty-todo delete 1
```

### Eliminar todas las tareas completadas

```bash
rusty-todo clean
```

## Estructura del proyecto

```
rusty-todo/
├── Cargo.toml          # Configuración y dependencias
├── src/
│   ├── main.rs         # CLI y lógica principal
│   ├── task.rs         # Modelo de tareas
│   └── storage.rs      # Persistencia JSON
└── README.md
```

## Dependencias

- **clap**: Framework CLI moderno con derive macros
- **serde/serde_json**: Serialización/deserialización JSON
- **anyhow**: Manejo de errores simplificado
- **chrono**: Manejo de fechas y timestamps

## Almacenamiento

Las tareas se guardan automáticamente en `rusty-todo.json` en el directorio actual donde ejecutes el comando, en formato JSON. El archivo se crea automáticamente la primera vez que agregas una tarea.

## Licencia

MIT


You can see my projects at https://renzobarcos.world/
