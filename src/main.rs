mod tasks;
use std::env;
use tasks::TaskList;

fn check_arg(n: usize) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let result = args.get(n);

    match result {
        Some(value) => return Some(value.to_string()),
        None => return None
    }
}

fn main() {
    let mut tasks = TaskList::load().expect("Cannot load the application");

    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];

    match arg1.as_str() {
        "add" => {
            let title = check_arg(2);
            match title {
                Some(t) => tasks.add(&t),
                None => println!("Please specifity the title")
            }
        }

        "delete" => {
            let id = check_arg(2);
            match id {
                Some(i) => {
                    match tasks.delete(i.parse::<usize>().unwrap()) {
                        Ok(t) => println!("Task {} deleted successfully", t.id),
                        Err(e) => println!("{}", e)
                    }
                },
                None => println!("Please specifity the id of task you want to delete")
            }
        }

        "list" => {
            tasks.list();
        }

        "get" => {
            let id: usize = check_arg(2).unwrap().parse().unwrap();
            let task = tasks.get_by_id(id);

            match task {
                Some(t) => {
                    println!("{t}")
                },
                None => {
                    println!("Task not found")
                }
            }
        }

        "mark-todo" => {
            let id = check_arg(2);

            match id {
                Some(i) => {
                    match tasks.mark_todo(i.parse::<usize>().unwrap()) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    }
                },
                None => println!("Please specify id of task you want to update")
            }
        }

        "mark-in-progress" => {
            let id = check_arg(2);

            match id {
                Some(i) => {
                    match tasks.mark_in_progress(i.parse::<usize>().unwrap()) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    }
                },
                None => println!("Please specify id of task you want to update")
            }
        }

        "mark-done" => {
            let id = check_arg(2);

            match id {
                Some(i) => {
                    match tasks.mark_done(i.parse::<usize>().unwrap()) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    }
                },
                None => println!("Please specify id of task you want to update")
            }
        }
        _ => {
            println!("Empty");
        }
    }

    tasks.save();
}
