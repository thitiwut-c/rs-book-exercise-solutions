use std::{
    collections::HashMap,
    io::{self},
};

#[derive(Debug)]
enum Command {
    AddDepartment,
    AddEmployee,
    ListEmployeesOfDepartment,
    ListAllEmployeesByDepartment,
    Exit,
}

fn main() {
    let mut department: HashMap<String, Vec<String>> = HashMap::new();
    department.insert(String::from("Engineering"), vec![String::from("Sally")]);
    department.insert(
        String::from("Sales"),
        vec![
            String::from("Amir"),
            String::from("Bob"),
            String::from("Charlie"),
        ],
    );

    let mut cmd_map = HashMap::new();
    cmd_map.insert("a", Command::AddDepartment);
    cmd_map.insert("b", Command::AddEmployee);
    cmd_map.insert("c", Command::ListEmployeesOfDepartment);
    cmd_map.insert("d", Command::ListAllEmployeesByDepartment);
    cmd_map.insert("z", Command::Exit);

    loop {
        println!("Enter command");
        println!("a) Add department");
        println!("b) Add employee");
        println!("c) List employees of a department");
        println!("d) List all employees by department");
        println!("z) Exit");

        let mut cmd_key = String::new();

        io::stdin()
            .read_line(&mut cmd_key)
            .expect("Failed read line");

        let cmd_key = cmd_key.trim();
        let cmd = match cmd_map.get(cmd_key) {
            Some(cmd) => cmd,
            None => {
                println!("Please enter a valid command");
                println!();
                continue;
            }
        };

        match cmd {
            Command::AddDepartment => {
                println!("Enter department name");

                let mut buf = String::new();
                io::stdin().read_line(&mut buf).expect("Failed read line");

                let department_name = buf.trim();

                match &department.get(department_name) {
                    Some(_) => println!("{} already exists", department_name),
                    None => {
                        department.insert(department_name.to_string(), Vec::new());
                        println!("{} added", department_name)
                    }
                }
            }
            Command::AddEmployee => {
                let dep_names = get_sorted_department_names(&department);
                println!("Select department number");
                loop {
                    for (i, dep_name) in dep_names.iter().enumerate() {
                        println!("{}) {}", i + 1, dep_name);
                    }

                    let mut buf = String::new();

                    io::stdin().read_line(&mut buf).expect("Failed read line");

                    let dep_idx: usize = match buf.trim().parse() {
                        Ok(idx) => idx,
                        Err(err) => panic!("{}", err),
                    };

                    let dep_name = match dep_names.get(dep_idx - 1) {
                        Some(name) => name.to_string(),
                        None => {
                            println!("Please enter a valid department number");
                            continue;
                        }
                    };

                    println!("Enter employee name");

                    io::stdin().read_line(&mut buf).expect("Failed read line");

                    let employee_name = buf.trim().to_string();

                    let employees = department.entry(dep_name.clone()).or_insert(Vec::new());
                    employees.push(employee_name.clone());

                    println!(
                        "{} has been added to {} department",
                        employee_name, dep_name
                    );

                    break;
                }
            }
            Command::ListEmployeesOfDepartment => {
                println!("Select department number");
                let dep_names = get_sorted_department_names(&department);
                for (i, dep_name) in dep_names.iter().enumerate() {
                    println!("{}) {}", i + 1, dep_name);
                }

                let mut buf = String::new();

                io::stdin().read_line(&mut buf).expect("Failed read line");

                let dep_idx: usize = match buf.trim().parse() {
                    Ok(idx) => idx,
                    Err(err) => panic!("{}", err),
                };

                let dep_name = match dep_names.get(dep_idx - 1) {
                    Some(name) => name.to_string(),
                    None => {
                        println!("Please enter a valid department number");
                        continue;
                    }
                };

                let employees = department.get(&dep_name).unwrap();

                println!("Employees in {} includes:", dep_name);
                println!("{}", employees.join(",\n"))
            }
            Command::ListAllEmployeesByDepartment => {
                for (dep_name, employees) in &department {
                    println!("{dep_name}:");

                    let mut employees = employees.clone();
                    employees.sort();

                    for employee in employees {
                        println!("\t{employee}");
                    }
                }
            }
            Command::Exit => break,
        }

        println!()
    }
}

fn get_sorted_department_names(d: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut names = Vec::new();
    for (name, _) in d {
        names.push(name.to_string())
    }

    names
}
