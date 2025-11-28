use rusqlite::{Connection, Result, params};
use crate::{Task, TaskPriority, TaskStatus};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("task-tracker.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            deadline TEXT,
            status TEXT,
            priority TEXT,
            creator_id INTEGER,
            executor_id INTEGER
        )",
        [],
    )?;
        Ok(conn)
}

pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (name, description, deadline, status, priority, creator_id, executor_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            task.name,
            task.description,
            task.deadline,
            task.status.to_string(),
            task.priority.to_string(),
            task.creator_id,
            task.executor_id
        ],
    )?;
    Ok(())
}

pub fn load_db(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM tasks"
    )?;

    let task_iter =stmt.query_map([], |row| {
        let id: u32 = row.get(0)?;
        let name: String = row.get(1)?;
        let description: String = row.get(2)?;
        let deadline: String = row.get(3)?;
        let status_str: String = row.get(4)?;
        let priority_str: String = row.get(5)?;
        let creator_id: u32 = row.get(6)?;
        let executor_id: u32 = row.get(7)?;

        let status = match status_str.as_str() {
            "unready" => TaskStatus::Unready,
            "in_process" => TaskStatus::InProcess,
            "done" => TaskStatus::Done,
            _ => TaskStatus::Undefined 
        };

        let priority = match priority_str.as_str() {
            "low" => TaskPriority::Low,
            "medium" => TaskPriority::Medium,
            "high" => TaskPriority::High,
            _ => TaskPriority::Undefined
        };
        Ok(Task {
            id,
            name,
            description,
            deadline,
            status,
            priority,
            creator_id,
            executor_id
        })
    })?;
    
    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}

pub fn update_task(
    conn: &Connection,
    id: u32,
    field: &str,
    value: &str
) -> Result<()> {
    let update_query = format!("UPDATE tasks SET {} = ?1 WHERE id = ?2", field);

    conn.execute(
        &update_query, 
        params![value, id],
    )?;

    Ok(())
}

pub fn delete_task(conn: &Connection, id: u32) -> Result<()> {
    let delete_query = "DELETE FROM tasks WHERE id = ?1";

    conn.execute(delete_query, params![id])?;

    Ok(())
}

