use std::cmp::Ordering;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct Task {
    pub id: usize,
    pub desc: String,
    pub status: Status,
}

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
        let status = format!(
            " {} ",
            match self.status {
                Status::None => {
                    "    "
                }
                Status::Done => {
                    "done"
                }
                Status::Prog => {
                    "prog"
                }
                Status::Urgent => {
                    "urge"
                }
            }
        );
        let full = format!("{}|{}| {}", id, status, self.desc);
        write!(
            f,
            "{}",
            match self.status {
                Status::None => full.blue(),
                Status::Done => full.strikethrough().green(),
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
