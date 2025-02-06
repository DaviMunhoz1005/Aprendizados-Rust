use std::collections::HashMap;

fn main() {
    let mut workers_departments = HashMap::new();
    let mut commands = Vec::new(); 
    commands.push(String::from("Add Sally to Engineering"));
    commands.push(String::from("Add Amir to Sales"));
    commands.push(String::from("Add Davi to Countbility"));
    commands.push(String::from("Add Cibele to Engineering"));
    commands.push(String::from("Add Lais to Engineering"));
    commands.push(String::from("Add Edgard to Countbility"));
    commands.push(String::from("Add Sabrina to Sales"));
    commands.push(String::from("Add Lavinia to Sales"));
    commands.push(String::from("Add Cleber to Engineering"));
    commands.push(String::from("Add Daniele to Sales"));

    process_commands(&mut commands, &mut workers_departments);

    display_departments(&mut workers_departments);
    println!();
    display_workers_in_department(&mut workers_departments, "Sales");
    println!();
    display_all_workers(&workers_departments);
}

fn process_commands(commands: &mut Vec<String>, workers_departments: &mut HashMap<String, Vec<String>>) {
    for command in commands {
        *command = command.replace("to", "");
        let split: Vec<&str> = command.split_whitespace().collect();
        if split[0] == "Add" {
            let name = split[1];
            let department = split[2];
            workers_departments.entry(department.to_string())
                .or_insert(Vec::new())
                .push(name.to_string());
        }
    }
}

fn display_departments(workers_departments: &mut HashMap<String, Vec<String>>) {
    for department in workers_departments.keys() {
        let number_of_workers_in_department = workers_departments.get(department).unwrap().len();
        println!("{} have {} workers", department, number_of_workers_in_department)
    }
}

fn display_workers_in_department(workers_departments: &HashMap<String, Vec<String>>, department_name: &str) {
    if let Some(workers) = workers_departments.get(department_name) {
        for worker_name in workers {
            println!("{} works in the {}", worker_name, department_name);
        }
    } 
}

fn display_all_workers(workers_departments: &HashMap<String, Vec<String>>) {
    let mut all_workers = Vec::new();
    for workers in workers_departments.values(){
        for worker in workers {
            all_workers.push(worker);
        }
    }
    all_workers.sort();
    for worker in  all_workers{
        println!("{} works here", worker);
    }
}