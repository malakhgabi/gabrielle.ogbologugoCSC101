use std::io;

fn main() {
    let mut input1 = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experienced = input1.trim().to_lowercase();

    let mut input2 = String::new();

    println!("How old are you?:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age: i32 = input2.trim().parse().expect("Not a valid number");

    if experienced == "no" {
        println!("Annual incentive = N100,000",);

    } else if experienced == "yes" {
        if age >= 40 {
            println!("Annual incentive = N1,560,000");

        } else if age >= 30 && age <= 39 {
            println!("Annual incentive = N1,300,000");

        } else if age < 28 {
            println!("Annual incentive = N1,300,00");
        }
    } else {
        println!("No incentive was specified for this age.",);
    } 
}
