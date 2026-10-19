use std::io::Write;

fn main() {

    let first_row = "33 Export,Legend,Maltina\n";
    let second_row = "Desperados,Turbo King,Amstel Malta\n";
    let third_row = "Goldberg,Williams,Malta Gold\n";
    let fourth_row = "Gulder,,Fayrouz\n";
    let fith_row = "Heineken,,\n";
    let sixth_row = "Star,,";

    let mut file = std::fs::File::create("Nigerian Breweries.csv").expect("create failed");
    file.write_all("Lager,Stout,Non-Alcoholic\n"
        .as_bytes()).expect("write failed");
    file.write_all(first_row.as_bytes()).expect("write failed");
    file.write_all(second_row.as_bytes()).expect("write failed");
    file.write_all(third_row.as_bytes()).expect("write failed");
    file.write_all(fourth_row.as_bytes()).expect("write failed");
    file.write_all(fith_row.as_bytes()).expect("write failed");
    file.write_all(sixth_row.as_bytes()).expect("write failed");

    println!("\nFile created for categories of drinks");
}
