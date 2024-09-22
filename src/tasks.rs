use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in-progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub status: TaskStatus,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.id, self.title, self.status)
    }
}

const FILE_PATH: &str = "./tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList(pub Vec<Task>);

impl Display for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Your tasks:")?;
        for task in self.0.iter() {
            writeln!(f, "{}", task)?;
        }
        Ok(())
    }
}

impl TaskList {
    pub fn load() -> Result<Self, String> {
        let mut file = match fs::File::open(FILE_PATH) {
            Ok(file) => file,
            Err(_) => {
                fs::File::create(FILE_PATH).expect("Failed to create tasks.json");
                fs::File::open(FILE_PATH).expect("Failed to open tasks.json")
            }
        };

        let mut content = String::new();
        file.read_to_string(&mut content).expect("Error");

        let tasks = serde_json::from_str(&content).unwrap_or_default();
        return Ok(Self(tasks));
    }

    pub fn save(&self) {
        let mut file = fs::File::create(FILE_PATH).expect("Failed to open tasks.json");
        let tasks = serde_json::to_string(&self).expect("Failed to get tasks list");

        let _ = file.write_all(tasks.as_bytes());
    }

    pub fn list(&self) {
        println!("{}", self);
    }

    pub fn get_by_id(&mut self, id: usize) -> Option<&mut Task> {
        // returns the position in vector
        let position = self.0.iter().position(|task| task.id == id);

        match position {
            Some(p) => Some(self.0.get_mut(p).unwrap()),
            None => None,
        }
    }

    pub fn add(&mut self, title: &str) {
        let mut id = self.0.len();
        id = id + 1;
        let new_task = Task {
            id,
            title: title.to_string(),
            status: TaskStatus::Todo,
        };

        self.0.push(new_task);
    }

    pub fn delete(&mut self, id: usize) -> Result<Task, String> {
        let task_position = self.0.iter().position(|t| t.id == id);
        match task_position {
            Some(pos) => {
                let task = self.0.remove(pos);
                Ok(task)
            }
            None => Err("Task with this id not found".to_string()),
        }
    }

    pub fn mark_todo(&mut self, id: usize) -> Result<&Task, String> {
        let task = self.get_by_id(id);

        match task {
            Some(t) => {
                t.status = TaskStatus::Todo;
                Ok(t)
            }
            None => Err("Task not found".to_string()),
        }
    }

    pub fn mark_in_progress(&mut self, id: usize) -> Result<&Task, String> {
        let task = self.get_by_id(id);

        match task {
            Some(t) => {
                t.status = TaskStatus::InProgress;
                Ok(t)
            }
            None => Err("Task not found".to_string()),
        }
    }

    pub fn mark_done(&mut self, id: usize) -> Result<&Task, String> {
        let task = self.get_by_id(id);

        match task {
            Some(t) => {
                t.status = TaskStatus::Done;
                Ok(t)
            }
            None => Err("Task not found".to_string()),
        }
    }
}
