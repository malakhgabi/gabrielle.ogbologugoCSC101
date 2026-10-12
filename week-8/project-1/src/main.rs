
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

    println!("Choose your profession:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    println!("Enter Your Choice [1-4]:");

    let mut profession = String::new();

    std::io::stdin().read_line(&mut profession).expect("Failed to read input");

    let choice: usize = profession.trim().parse().expect("Enter a valid number");

    println!("Enter years of experience:");

    let mut experience = String::new();

    std::io::stdin().read_line(&mut experience).expect("Failed to read input");

    let years: u32 = experience.trim().parse().expect("Enter a valid number of years");

    let index = if years <= 2 {
        0
    } else if years <= 5 {
        1
    } else if years <= 8 {
        2
    } else if years <= 10 {
        3
    } else if years <= 13 {
        4
    } else {
        5
    };

    if choice < 1 || choice > 4 {
        println!("Invalid profession choice.");
        return;
    }

    if years > 13 {
        println!("SES is the highest listed category.");
    }

    let position = if choice == 1 {
        office_admin[index]
    } else if choice == 2 {
        academic[index]
    } else if choice == 3 {
        lawyer[index]
    } else {
        teacher[index]
    };

    println!("\n--- RESULT ---");
    println!("Years of experience: {}", years);
    println!("APS Level: {}", levels[index]);
    println!("Position: {}", position);
}
