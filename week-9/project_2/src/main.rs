use std::io::Write;

fn main() {
    let students: Vec<(&str, &str, &str, u64)>= vec![
    ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
    ("Adams Aliyu", "ECO10110101", "Economics", 100),
    ("Shania Blonde", "CSC10328828", "Computer", 200),
    ("Adekunle Gold", "EEE11020202", "Electrical", 200),
    ("Blanca Edemoh", "MEE10202001", "Mechanical", 100)
];

println!("Student Name\tMatric Number\tDepartment\tLevel");

for student in &students {
    println!("{}\t{}\t{}\t{}",
        student.0, student.1, student.2, student.3 
        );
}

let mut file = std::fs::File::create("PAU_SMIS.csv").expect("Could not create the File");

file.write_all("Student Name,Matric Number,Department,Level\n".as_bytes()).expect("write failed");

for student in &students {

let record = format! ("{},{},{},{}\n",
    student.0, student.1, student.2, student.3
    );

file.write_all(record.as_bytes())
.expect("Write failed");
}

println!("Students details have been saved!");
}


