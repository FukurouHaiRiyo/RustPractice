mod back_code;
use back_code::BackCode;
use std::io::{self, Write};

fn main() {
    let mut back_code = BackCode::new();
    let mut input = String::new();
    let stdin = io::stdin();

    loop{
        println!("What do you want to do? 1-add, 2-remove, 3-edit, 4-exit");
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let choice: i32 = input.trim().parse().expect("Please type a number!");

        match choice {
            1 => { // add tasks
                println!("How many tasks do you want to add?");
                input.clear();
                stdin.read_line(&mut input).expect("Failed to read line");
                let nr: usize = input.trim().parse().expect("Please ener a number");

                for _ in 0..nr{
                    println!("Enter the task");
                    input.clear();
                    stdin.read_line(&mut input).expect("Failed to read line");
                    back_code.add_capsule_stuff(input.trim().to_string());
                }

                // print the array
                println!("The tasks are:");
                for (i, task) in back_code.get_capsule_stuff().iter().enumerate(){
                    println!("{}: {}", i, task);
                }
            },

            2 => { // remove tasks
                println!("Which task do you want to remove?");
                input.clear();
                stdin.read_line(&mut input).expect("Failed to read line");
                let remove: usize = input.trim().parse().expect("Please type a number!");
                back_code.remove_capsule_stuff(remove);
            },

            3 => { // edit tasks
                if back_code.get_capsule_stuff().len() == 0{
                    println!("There are no tasks to edit");
                } else {
                    // print the array
                    println!("The tasks are:");
                    for (i, task) in back_code.get_capsule_stuff().iter().enumerate(){
                        println!("{}: {}", i, task);
                    }

                    println!("Which task do you want to edit?");
                    input.clear();
                    stdin.read_line(&mut input).expect("Failed to read line");
                    let edit: usize = input.trim().parse().expect("Please type a number!");

                    println!("Enter the new task");
                    input.clear();
                    stdin.read_line(&mut input).expect("Failed to read line");
                    back_code.edit_capsule_stuff(edit, input.trim().to_string());

                    // print the array
                    println!("The tasks are:");
                    for (i, task) in back_code.get_capsule_stuff().iter().enumerate(){
                        println!("{}: {}", i, task);
                    }
                }
            }

            4 => { // exit
                println!("Goodbye!");
                break;
            },

            _ => {
                println!("Invalid choice, try again (valid choices are 1, 2, 3, 4)");
            }
        }
    }
}
