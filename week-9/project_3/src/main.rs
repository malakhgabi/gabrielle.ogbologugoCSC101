use std::io::Write;

fn main() {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorochaa Cdistus Ogbonna",
        "Adewedde Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file =std::fs::File::create("EFCC_Merge.txt") .expect("Could not create file");

    for i in 0..commissioners.len() {
        let record = format!(
            "{} | {} | {}\n",
            commissioners[i], ministries[i], zones[i]
        );

        println!("{}", record);

        file.write_all(record.as_bytes())
            .expect("Could not write to file");
    }

    println!("All records merged successfully!");
}
