use std::io::Read;

fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main() {
    println!("Select your role:");
    println!("1. Administrator");
    println!("2. Project manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let role = input.trim();

    if role == "1" {
        administrator();
    } else if role == "2" {
        project_manager();
    } else if role == "3" {
        employee();
    } else if role == "4" {
        customer();
    } else if role == "5" {
        vendor();
    } else {
        println!("Invalid role selected.");
    }
}
