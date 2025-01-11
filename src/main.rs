use colored::{ColoredString, Colorize};
use dodo::*;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

fn main() {
    let cli = Cli::parse_args();

    let mut app = App::load().unwrap_or_else(|_| App::default());
    app.update_counts();

    match cli.command {
        Some(Commands::Add { task }) => {
            if let Some(task_vec) = task {
                let task = app.add(task_vec);
                println!();
                println!("Added to dodo: ");
                println!("==> {}", task.desc);
            }
        }
        Some(Commands::Done { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.done(id) {
                        Ok(_t) => {
                            println!();
                            println!("Marked as done: ");
                            println!("==> {}", _t.desc);
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        Some(Commands::Prog { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.prog(id) {
                        Ok(_t) => {
                            println!();
                            println!("{}", "Marked as in progress:".yellow());
                            println!("==> {}", _t.desc);
                        }
                        Err(e) => app.print_err(id, e)
                    }
                }
            }
        }
        Some(Commands::Urge { id }) => {
            if let Some(ids) = id {
                for id in ids {
                    match app.urge(id) {
                        Ok(_t) => {
                            println!();
                            println!("{}", "Marked as urgent:".red());
                            println!("==> {}", _t.desc);
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
                        Ok(_t) => {
                            println!();
                            println!("Marked as normal:");
                            println!("==> {}", _t.desc);
                        }
                        Err(e) => app.print_err(id, e),
                    }
                }
            }
        }
        // Some(Commands::Up { id, count }) => {
        //     if let Some(id) = id {
        //         if let Some(count) = count {
        //             match app.move_up(id, count) {
        //                 Ok(t) => {
        //                     println!();
        //                     println!("Task moved up by {}:", count);
        //                     println!();
        //                     println!("==> {}", t.desc);
        //                 }
        //                 Err(e) => app.print_err(id, e),
        //             }
        //         } else {
        //             match app.move_up(id, 1) {
        //                 Ok(t) => {
        //                     println!();
        //                     println!("Task moved up by 1:");
        //                     println!();
        //                     println!("==> {}", t.desc);
        //                 }
        //                 Err(e) => app.print_err(id, e),
        //             }
        //         }
        //     }
        // }
        // Some(Commands::Down { id, count }) => {
        //     if let Some(id) = id {
        //         if let Some(count) = count {
        //             match app.move_down(id, count) {
        //                 Ok(t) => {
        //                     println!();
        //                     println!("Task moved down by {}:", count);
        //                     println!();
        //                     println!("==> {}", t.desc);
        //                 }
        //                 Err(e) => app.print_err(id, e),
        //             }
        //         } else {
        //             match app.move_down(id, 1) {
        //                 Ok(t) => {
        //                     println!();
        //                     println!("Task moved down by 1:");
        //                     println!();
        //                     println!("==> {}", t.desc);
        //                 }
        //                 Err(e) => app.print_err(id, e),
        //             }
        //         }
        //     }
        // }
        Some(Commands::Remove { id }) => {
            if let Some(ids) = id {
                // Allow removal of all tasks
                let wild = ids.join("").trim().to_lowercase();
                if wild == "all" {
                    let mut s = String::new();
                    print!("{} [Y/n]: ","Are you sure you want to remove all tasks?".red().underline());
                    let _ = stdout().flush();
                    stdin().read_line(&mut s).expect("Could not read input");
                    if let Some('\n') = s.chars().next_back() { s.pop(); }
                    if let Some('\r') = s.chars().next_back() { s.pop(); }
                    match s.as_str() {
                        "Y" => {
                            println!("Removing all tasks");
                            app = App::default();
                        }
                        "y" => println!("Must be uppercase! Ignoring `y`..."),
                        "n" => {},
                        _ => println!("Ignoring `{}`...", s)
                    }
                } else {
                    // Parse ourselves because we want to accept `all` as a possible input
                    for id in ids {
                        let id: usize = match id.parse() {
                            Ok(id) => id,
                            Err(_) => {
                                println!("{} {}","Invalid ID:".red(), id);
                                continue
                            },
                        };
                        match app.remove(id) {
                            Some(t) => {
                                println!();
                                println!("{}", "Removed:".underline().red());
                                println!();
                                println!("==> {}", t.desc);
                            }
                            None => {
                                app.print_err(id, LoadError::OutOfBounds);
                            }
                        }
                    }
                }
            }
        }
        Some(Commands::Reset) => {
            println!();
            if !app.tasks.is_empty() {
                for (index, task) in app.tasks.iter_mut().enumerate() {
                    task.set_id(index)
                }
                println!("{}", "Task ID reset complete...".green());
            } else {
                println!("{}", "There are no tasks to reset".red());
            }
        }
        None => {}
    }
    print_tasks(&app.tasks);
    app.save().unwrap();
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
