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
        let mut task = Task::new(desc);

        let id = if self.tasks.is_empty() {
            0
        } else {
            self.tasks.iter().max().unwrap().id + 1
        };

        task.set_id(id);

        for i in 0..self.tasks.len() {
            if self.tasks[i].status != Status::Prog 
            && self.tasks[i].status != Status::Urgent {
                self.tasks.insert(i, task.clone());
                break;
            } 
        }
        task
    }

    pub fn pos_from_id(&self, id: usize) -> Option<usize> {
        self.tasks.iter().position(|task| task.id == id)
    }

    pub fn remove(&mut self, id: usize) -> Option<Task> {
        if let Some(index) = self.pos_from_id(id) {
            self.tasks.remove(index)
        } else {
            None
        }
    }

    pub fn done(&mut self, id: usize) -> Result<Task, LoadError> {
        if let Some(index) = self.pos_from_id(id) {
            match self.tasks.get_mut(index) {
                Some(t) => {
                    t.status = Status::Done;
                }
                None => return Err(LoadError::OutOfBounds),
            }
            let task = self.tasks.remove(index).unwrap();
            self.tasks.push_back(task.clone());
            Ok(task)
        } else {
            Err(LoadError::OutOfBounds)
        }
    }

    pub fn prog(&mut self, id: usize) -> Result<Task, LoadError> {
        if let Some(index) = self.pos_from_id(id) {
            match self.tasks.get_mut(index) {
                Some(t) => {
                    t.status = Status::Prog;
                }
                None => return Err(LoadError::OutOfBounds),
            }
            let task = self.tasks.remove(index).unwrap();
            self.tasks.push_front(task.clone());
            Ok(task)
        } else {
            Err(LoadError::OutOfBounds)
        }
    }
    pub fn urge(&mut self, id: usize) -> Result<Task, LoadError> {
        if let Some(index) = self.pos_from_id(id) {
            match self.tasks.get_mut(index) {
                Some(t) => {
                    t.status = Status::Urgent;
                }
                None => return Err(LoadError::OutOfBounds),
            }
            let task = self.tasks.remove(index).unwrap();
            for i in 0..self.tasks.len() {
                if self.tasks[i].status != Status::Prog {
                    self.tasks.insert(i, task.clone());
                    break;
                }
            }
            Ok(task)
        } else {
            Err(LoadError::OutOfBounds)
        }
    }
    
    pub fn norm(&mut self, id: usize) -> Result<Task, LoadError> {
        if let Some(index) = self.pos_from_id(id) {
            match self.tasks.get_mut(index) {
                Some(t) => {
                    t.status = Status::None;
                }
                None => return Err(LoadError::OutOfBounds),
            }

            let task = self.tasks.remove(index).unwrap();
            for i in 0..self.tasks.len() {
                if self.tasks[i].status != Status::Prog
                && self.tasks[i].status != Status::Urgent {
                    self.tasks.insert(i, task.clone());
                    break;
                }
            }
            Ok(task)
        } else {
            Err(LoadError::OutOfBounds)
        }
    }

    pub fn move_up(&mut self, id: usize, count: usize) -> Result<Task, LoadError> {
        if let Some(index) = self.pos_from_id(id) {
            let mov = self.tasks.remove(index);
            if let Some(mov) = mov {
                let new_index = match index as isize - count as isize >= 0 {
                    true => index - count,
                    false => 0,
                };
                self.tasks.insert(new_index, mov.clone());
                Ok(mov)
            } else {
                Err(LoadError::OutOfBounds)
            }
        } else {
            Err(LoadError::OutOfBounds)
        }
    }

    pub fn move_down(&mut self, id: usize, count: usize) -> Result<Task, LoadError> {
        if let Some(index) = self.pos_from_id(id) {
            let mov = self.tasks.remove(index);
            if let Some(mov) = mov {
                if index + count < self.tasks.len() {
                    self.tasks.insert(index + count, mov.clone());
                    Ok(mov)
                } else {
                    self.tasks.push_back(mov.clone());
                    Ok(mov)
                }
            } else {
                Err(LoadError::OutOfBounds)
            }
        } else {
            Err(LoadError::OutOfBounds)
        }
    }

    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "subora", "dodo")
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
        println!("{}{}: task {}", e, "Error".red(), id);
        println!();
    }
}
