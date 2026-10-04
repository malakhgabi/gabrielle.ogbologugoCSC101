use std::io;

fn main() {
    println!("===== RESTAURANT MENU =====");
    println!("P - Poundo Yam / Edikankong Soup - N3200");
    println!("F - Fried Rice & Chicken        - N3000");
    println!("A - Amala & Ewedu Soup          - N2500");
    println!("E - Eba & Egusi Soup            - N2000");
    println!("W - White Rice & Stew            - N2500");

    println!("Enter food type:");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");

    let food = food.trim().to_uppercase();

    println!("Enter quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");

    let quantity: i32 = quantity.trim().parse().expect("Please enter a valid number");

    let mut price: i32=0;

    if food == "P" {
        price = 3200;

    } else if food == "F" {
        price = 3000;

    } else if food == "A" {
        price = 2500;

    } else if food == "E" {
        price = 2000;

    } else if food == "W" {
        price = 2500;

    } else {
        println!("Invalid food type!");
    }

    let total = price * quantity;

    println!("Total before discount: N{}", total);

    if total > 10000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!("Discount: N{}", discount);
        println!("Final amount to pay: N{}", final_total);
    } else {
        println!("No discount.");
        println!("Final amount to pay: N{}", total);
    }
}