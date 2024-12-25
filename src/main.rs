use colored::{ColoredString, Colorize};
use dodo::*;
use std::collections::VecDeque;

fn main() {
    let cli = Cli::parse_args();

    let mut app = match App::load() {
        Ok(v) => v,
        Err(_) => App::default(),
    };

    for (id, task) in app.tasks.iter_mut().enumerate() {
        task.set_id(id);
    }

    match cli.command {
        Some(Commands::Add { task }) => {
            if let Some(task_vec) = task {
                let task = app.add(task_vec);
                println!();
                println!("Added to dodo: ");
                println!();
                println!("==> {}", task.desc);
                app.clone().save().unwrap();
            }
        }
        Some(Commands::Done { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.done(id) {
                        Ok(t) => {
                            println!();
                            println!("Marked as done: ");
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        Some(Commands::Urge { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.urge(id) {
                        Ok(t) => {
                            println!();
                            println!("Marked as urgent:");
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        Some(Commands::Norm { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.norm(id) {
                        Ok(t) => {
                            println!();
                            println!("Marked as normal:");
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        Some(Commands::Up { id, count }) => {
            if let Some(id) = id {
                if let Some(count) = count {
                    match app.move_up(id, count) {
                        Ok(t) => {
                            println!();
                            println!("Task moved up by {}:", count);
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                } else {
                    match app.move_up(id, 1) {
                        Ok(t) => {
                            println!();
                            println!("Task moved up by 1:");
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        Some(Commands::Down { id, count }) => {
            if let Some(id) = id {
                if let Some(count) = count {
                    match app.move_down(id, count) {
                        Ok(t) => {
                            println!();
                            println!("Task moved down by {}:", count);
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                } else {
                    match app.move_down(id, 1) {
                        Ok(t) => {
                            println!();
                            println!("Task moved down by 1:");
                            println!();
                            println!("==> {}", t.desc);
                            app.clone().save().unwrap();
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        Some(Commands::Remove { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.remove(id) {
                        Some(t) => {
                            println!();
                            println!("{}", "Removed: ".red());
                            println!();
                            println!("{}", t.desc);
                            app.clone().save().unwrap();
                        }
                        None => {
                            app.print_err(id, LoadError::OutOfBounds);
                        }
                    }
                }
            }
        }
        None => {}
    }

    for (id, task) in app.tasks.iter_mut().enumerate() {
        task.set_id(id);
    }

    print_tasks(&app.tasks);
}

fn print_tasks(tasks: &VecDeque<Task>) {
    println!();
    println!("{}", head());
    println!("{}", body());
    for task in tasks {
        println!("{}", task);
    }
    println!();
}

fn head() -> ColoredString {
    " id | stat | task ".blue()
}

fn body() -> ColoredString {
    "====|======|======>".blue()
}
