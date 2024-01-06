extern crate colorful;

use colorful::Color;
use colorful::Colorful;
use std::collections::HashMap;
use std::io;

#[derive(PartialEq)]
enum Command {
    Add { department: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn add_employee_to_department(
        department: &str,
        name: &str,
        company: &mut HashMap<String, Vec<String>>,
    ) {
        company
            .entry(department.to_string())
            .or_insert_with(Vec::new)
            .push(name.to_string());
    }

    fn get_company_department_people(company: &mut HashMap<String, Vec<String>>) {
        for (department, people) in company.iter() {
            println!(
                "{}",
                format!("Department: {}", department).color(Color::Magenta)
            );
            for person in people {
                println!("{}", format!("Person: {}", person).color(Color::Green));
            }
        }
    }

    fn get_department_people(company: &HashMap<String, Vec<String>>, department: &str) {
        match company.get(department) {
            Some(people) => {
                for person in people {
                    println!("{}", format!("Person: {}", person).color(Color::Green));
                }
            }
            None => println!("{}", "Department not found".color(Color::Red)),
        }
    }

    fn quit_command() {
        println!("{}", "Quitting".color(Color::Yellow));
    }

    fn new(prompt: &str) -> Result<Command, &'static str> {
        let command_pieces = prompt.split_whitespace().collect::<Vec<&str>>();
        let command_prompt = command_pieces.first().expect("Invalid command");

        match command_prompt {
            &"Add" => Ok(Command::parse_add_command(&command_pieces)),
            &"List" => Command::parse_list_command(&command_pieces),
            &"All" => Ok(Command::All),
            &"Quit" => Ok(Command::Quit),
            _ => Err("Invalid command"),
        }
    }

    fn parse_add_command(command_pieces: &[&str]) -> Command {
        let department = command_pieces
            .get(3)
            .expect("Missing department name in 'Add' command");
        let name = command_pieces
            .get(1)
            .expect("Missing employee name in 'Add' command");
        Command::Add {
            department: department.to_string(),
            name: name.to_string(),
        }
    }

    fn parse_list_command(command_pieces: &[&str]) -> Result<Command, &'static str> {
        if let Some(department) = command_pieces.get(1) {
            Ok(Command::List(department.to_string()))
        } else {
            Err("Missing department name in 'List' command")
        }
    }

    fn execute(&self, company: &mut HashMap<String, Vec<String>>) {
        match self {
            Command::Add { department, name } => {
                Command::add_employee_to_department(department, name, company);
                println!(
                    "{}",
                    format!("Added {} to {}", name, department).color(Color::Cyan)
                );
            }
            Command::List(department) => {
                Command::get_department_people(company, department);
            }
            Command::All => {
                Command::get_company_department_people(company);
            }
            Command::Quit => {
                Command::quit_command();
            }
        }
    }

    fn command_menu() {
        println!(
            "{}",
            "Type 'Add <name> to <department>' to add an employee".color(Color::Yellow)
        );
        println!(
            "{}",
            "Type 'List <department>' to list the employees of a department".color(Color::Yellow)
        );
        println!(
            "{}",
            "Type 'All' to list all employees by department".color(Color::Yellow)
        );
        println!("{}", "Type 'Quit' to quit".color(Color::Yellow));
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();

    loop {
        let mut input = String::new();

        Command::command_menu();

        stdin.read_line(&mut input).unwrap();

        let command = Command::new(&input);

        match command {
            Ok(cmd) => {
                if cmd == Command::Quit {
                    break;
                }
                cmd.execute(&mut company);
            }
            Err(err) => println!("{}", err.color(Color::Red)),
        }
    }
}
