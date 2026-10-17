
use std::io;

fn main() {
    let mut candidates: Vec<(String, u32)> = Vec::new();

    loop {
        let mut name = String::new();

        println!("Enter candidate name or type done to finish:");
        io::stdin().read_line(&mut name).expect("Error reading name");

        let name = name.trim().to_string();

        if name.to_lowercase() == "done" {
            break;
        }

        let mut years = String::new();

        println!("Enter years of experience:");
        io::stdin().read_line(&mut years).expect("Error reading years");

        let years: u32 = years.trim().parse().expect("Enter a valid number");

        candidates.push((name, years));
    }

    if candidates.len() == 0 {
        println!("No candidates entered.");
    } else {
        let mut highest = 0;

        for i in 0..candidates.len() {
            if candidates[i].1 > candidates[highest].1 {
                highest = i;
            }
        }

        println!("Candidate with the highest experience is {}", candidates[highest].0);
        println!("Years of experience: {}", candidates[highest].1);
    }
}
