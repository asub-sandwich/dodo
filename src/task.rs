use std::cmp::Ordering;
use colored::Colorize;
use console::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct Task {
    pub id: usize,
    //unique: usize?, // This would be the `relate` field
    pub desc: String,
    pub status: Status,
}

/*

pub struct Subtask {
    pub sub_id: usize,
    pub task_id: usize,
    pub desc: String,
    pub status: Status
}

*/

impl Task {
    pub fn new(desc: String) -> Self {
        Self {
            id: 0,
            desc,
            status: Status::None,
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id
            .cmp(&other.id)
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_len = (0..).take_while(|i| 10usize.pow(*i) <= self.id).count();
        let id = format!(" {}{} ", self.id, if id_len < 2 { " " } else { "" });

        let done = match unicode_width::UnicodeWidthChar::width('🐸') {
            None => unreachable!("we checked that there is at least one character"),
            Some(width) => {
                match width {
                    1 => " 🐸🐸 ",
                    2 => "🐸🐸",
                    _ => unreachable!("we checked that there is at least one character"),
                }
            }
        };

        let prog = match unicode_width::UnicodeWidthChar::width('🦤') {
            None => unreachable!("we checked that there is at least one character"),
            Some(width) => {
                match width {
                    1 => " 🦤🦤 ",
                    2 => "🦤🦤",
                    _ => unreachable!("we checked that there is at least one character"),
                }
            }
        };

        let urge = match unicode_width::UnicodeWidthChar::width('🚨') {
            None => unreachable!("we checked that there is at least one character"),
            Some(width) => {
                match width {
                    1 => " 🚨🚨 ",
                    2 => "🚨🚨",
                    _ => unreachable!("we checked that there is at least one character"),
                }
            }
        };

        let status = format!(
            " {} ",
            match self.status {
                Status::None => {
                    Emoji("    ", "    ")
                }
                Status::Done => {
                    Emoji(done,"done")
                }
                Status::Prog => {
                    Emoji(prog, "prog")
                }
                Status::Urgent => {
                    Emoji(urge, "urge")
                }
            }
        );
        let full = format!("{}|{}| {}", id, status, self.desc);
        write!(
            f,
            "{}",
            match self.status {
                Status::None => full.blue(),
                Status::Done => full.underline().green(),
                Status::Prog => full.underline().yellow(),
                Status::Urgent => full.underline().red(),
            }
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    #[default]
    None = 0,
    Done = 1,
    Prog = 2,
    Urgent = 3,
}
