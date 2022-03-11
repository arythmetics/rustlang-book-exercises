use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use itertools::Itertools;


// todos
// Add error handler in show department if the department doesn't exist

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    
    let mut user_input = String::new();

    loop {
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.contains("show") {
            display_handler(&user_input, &mut company)
        }
        
        else if user_input.contains("add") {
            let vec: Vec<String> = parse_emp_input(&user_input);
            add_employee(&vec[0], &vec[1], &mut company);
        }

        else {
            println!("Unknown command. The program knows 'add' or 'show'")
        }

        user_input.clear();
    } 
}

fn parse_emp_input(user_input: &String) -> Vec<String>{
    
    let edited_input: String = user_input.replace("add ", "").replace("to ", "");

    let split_str: Vec<String> = edited_input.split_whitespace().map(str::to_string).collect();
    return split_str
}

fn add_employee(name: &String, department: &String, company_db: &mut HashMap<String, Vec<String>>) {
    // Add department if it's not there
    match company_db.entry(department.to_string()) {
        Entry::Occupied(mut entry) => { entry.get_mut().push(name.to_string()); },
        Entry::Vacant(entry) => { entry.insert(vec!(name.to_string())); },
    }
}

fn display_handler(user_input: &String, company: &mut HashMap<String, Vec<String>>) {
    // Takes the input for the show command and handles company or department printing
    if user_input.contains("department") {
        let department: String = user_input.split_whitespace().last().map(str::to_string).unwrap();
        get_department(&department, company)
    }

    else if user_input.contains("company") {
        for department in company.keys().sorted() {
            get_department(&department, company)
        }
    }

    else {
        println!("Unknown command. You can show either company or department");
    }
}

fn get_department(department: &String, company: &HashMap<String, Vec<String>>) {
    
    if company.contains_key(department) {
        let mut department_employees: Vec<String> = company.get(department).unwrap().clone();
        department_employees.sort();
        println!("{}:{:?}", department, department_employees)
    }
    
    else {
        println!("Unknown department. These are the ones available:");
        println!("{:?}", company.keys())
    }
}
