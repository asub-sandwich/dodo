use crate::*;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct App {
    pub tasks: VecDeque<Task>,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
    OutOfBounds,
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::File => "File".red(),
            Self::Format => "Format".red(),
            Self::OutOfBounds => "OutOfBounds".red(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum SaveError {
    File,
    Write,
    Format,
}

impl App {
    pub fn add(&mut self, desc_vec: Vec<String>) -> Task {
        let desc = desc_vec.join(" ");
        let task = Task::new(desc);
        self.tasks.push_back(task.clone());
        task
    }

    pub fn remove(&mut self, id: usize) -> Option<Task> {
        self.tasks.remove(id)
    }

    pub fn done(&mut self, id: usize) -> Result<Task, LoadError> {
        match self.tasks.get_mut(id) {
            Some(t) => {
                t.status = Status::Done;
                return Ok(t.clone());
            }
            None => return Err(LoadError::OutOfBounds),
        }
    }

    pub fn urge(&mut self, id: usize) -> Result<Task, LoadError> {
        match self.tasks.get_mut(id) {
            Some(t) => {
                t.status = Status::Urgent;
                return Ok(t.clone());
            }
            None => return Err(LoadError::OutOfBounds),
        }
    }

    pub fn norm(&mut self, id: usize) -> Result<Task, LoadError> {
        match self.tasks.get_mut(id) {
            Some(t) => {
                t.status = Status::None;
                return Ok(t.clone());
            }
            None => return Err(LoadError::OutOfBounds),
        }
    }

    pub fn move_up(&mut self, id: usize, count: usize) -> Result<Task, LoadError> {
        let mov = self.tasks.remove(id);

        if let Some(mov) = mov {
            let index = match id as isize - count as isize >= 0 {
                true => id - count,
                false => 0,
            };
            self.tasks.insert(index, mov.clone());
            Ok(mov)
        } else {
            return Err(LoadError::OutOfBounds);
        }
    }

    pub fn move_down(&mut self, id: usize, count: usize) -> Result<Task, LoadError> {
        let mov = self.tasks.remove(id);

        if let Some(mov) = mov {
            if id + count < self.tasks.len() {
                self.tasks.insert(id + count, mov.clone());
                Ok(mov)
            } else {
                self.tasks.push_back(mov.clone());
                Ok(mov)
            }
        } else {
            Err(LoadError::File)
        }
    }

    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "subora", "fuck")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("dodo.json");
        path
    }

    pub fn load() -> Result<App, LoadError> {
        use std::io::prelude::*;

        let mut contents = String::new();

        let mut file = std::fs::File::open(Self::path()).map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    pub fn save(self) -> Result<(), SaveError> {
        use std::io::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|_| SaveError::File)?;
        }
        {
            let mut file = std::fs::File::create(path).map_err(|_| SaveError::File)?;
            file.write_all(json.as_bytes())
                .map_err(|_| SaveError::Write)?;
            Ok(())
        }
    }

    pub fn print_err(&self, id: usize, e: LoadError) {
        println!();
        println!("{} {}: Couldn't move up task {}", e, "Error".red(), id);
        println!();
    }
}
