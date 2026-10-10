
use std::io;

fn get_input() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// 1. Area of a trapezium
fn trapezium_area() {
    println!("Enter height:");
    let height = get_input();

    println!("Enter first base:");
    let base1 = get_input();

    println!("Enter second base:");
    let base2 = get_input();

    let area = height / 2.0 * (base1 + base2);
    println!("Area of trapezium = {}", area);
}

// 2. Area of a rhombus
fn rhombus_area() {
    println!("Enter first diagonal:");
    let diagonal1 = get_input();

    println!("Enter second diagonal:");
    let diagonal2 = get_input();

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of rhombus = {}", area);
}

// 3. Area of a parallelogram
fn parallelogram_area() {
    println!("Enter base:");
    let base = get_input();

    println!("Enter altitude:");
    let altitude = get_input();

    let area = base * altitude;
    println!("Area of parallelogram = {}", area);
}

// 4. Surface area of a cube
fn cube_surface_area() {
    println!("Enter side length:");
    let side = get_input();

    let area = 6.0 * side * side;
    println!("Surface area of cube = {}", area);
}

// 5. Volume of a cylinder
fn cylinder_volume() {
    println!("Enter radius:");
    let radius = get_input();

    println!("Enter height:");
    let height = get_input();

    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of cylinder = {}", volume);
}

fn main() {
    println!("===== SHAPE CALCULATOR =====");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Surface Area of a Cube");
    println!("5. Volume of a Cylinder");

    println!("Choose a shape (1-5):");

    let choice = get_input();

    if choice == 1.0 {
        trapezium_area();
    } else if choice == 2.0 {
        rhombus_area();
    } else if choice == 3.0 {
        parallelogram_area();
    } else if choice == 4.0 {
        cube_surface_area();
    } else if choice == 5.0 {
        cylinder_volume();
    } else {
        println!("Invalid choice. Please select a number from 1 to 5.");
    }
}
