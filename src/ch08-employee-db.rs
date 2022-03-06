use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    
    let mut user_input = String::new();

    loop {
        io::stdin().read_line(&mut user_input).unwrap();

        // Parse user input into a vector of name and department
        let vec: Vec<String> = parse_input(&user_input);
        add_employee(&vec[0], &vec[1], &mut company);

        user_input.clear();
        for (key, value) in &company {
            println!("{}: {:?}", key, value);
        }
    } 
}

fn parse_input(user_input: &String) -> Vec<String>{
    
    let edited_input: String = user_input.replace("add", "").replace("to", "");

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
