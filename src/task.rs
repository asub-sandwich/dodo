use colored::Colorize;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
            status: Status::None
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl std::fmt::Display for Task {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_len = (0..).take_while(|i| 10usize.pow(*i) <= self.id).count();
        let id = format!(" {}{} ", self.id, if id_len < 2 { " " } else { "" });
        let status = format!(" {} ", match self.status {
            Status::None => { "    " },
            Status::Done => { "done" },
            Status::Urgent => { "urge" },
        });
        let full = format!("{}|{}| {}", id, status, self.desc);
        write!(f,"{}", match self.status {
            Status::None => full.blue(),
            Status::Done => full.strikethrough().green(),
            Status::Urgent => full.underline().red(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    None,
    Done,
    Urgent,
}