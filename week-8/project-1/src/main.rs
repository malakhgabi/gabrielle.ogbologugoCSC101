fn main() {
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "-",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    println!("APS LEVEL CHECKER");
    println!("-----------------");

    println!("Choose a profession:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let mut profession = String::new();

    println!("Enter your choice (1-4):");

    std::io::stdin().read_line(&mut profession).expect("Failed to read input");

    let choice: usize = profession.trim().parse().expect("Please enter a valid number");

    println!("Enter the APS level (1-6):");

    let mut input_level = String::new();

    std::io::stdin().read_line(&mut input_level).expect("Failed to read input");

    let level: usize = input_level.trim().parse().expect("Please enter a valid number");

    if level < 1 || level > 6 {
        println!("Invalid APS level.");
        return;
    }

    let index = level - 1;

    if choice == 1 {
        println!("Profession: Office Admin");
        println!("Position: {}", office_admin[index]);
    } else if choice == 2 {
        println!("Profession: Academic");
        println!("Position: {}", academic[index]);
    } else if choice == 3 {
        println!("Profession: Lawyer");
        println!("Position: {}", lawyer[index]);
    } else if choice == 4 {
        println!("Profession: Teacher");
        println!("Position: {}", teacher[index]);
    } else {
        println!("Invalid profession choice.");
        return;
    }

    println!("APS Level: {}", levels[index]);
}