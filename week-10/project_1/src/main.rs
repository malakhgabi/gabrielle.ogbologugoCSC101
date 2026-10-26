struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

impl Laptop {
    fn cost(&self) -> u32 {
        self.price * self.quantity
    }
}

fn main() {
    let hp = Laptop {
        brand:String::from("HP"),
        price:650000,
        quantity:3,
    };
    
    let ibm = Laptop {
        brand:String::from("IBM"),
        price:755000,
        quantity:3,
    };
    
    let toshiba = Laptop {
        brand:String::from("Toshiba"),
        price:550000,
        quantity:3,
    };
    
    let dell = Laptop {
        brand:String::from("Dell"),
        price:850000,
        quantity:3,
    };

    
    println!("--- Purchase Of 3 Laptop Brands ---");
    println!("Cost for 3 {} laptops: ₦{}", hp.brand, hp.cost());
    println!("Cost for 3 {} laptops: ₦{}", ibm.brand, ibm.cost());
    println!("Cost for 3 {} laptops: ₦{}", toshiba.brand, toshiba.cost());
    println!("Cost for 3 {} laptops: ₦{}", dell.brand, dell.cost());
    println!("-----------------------------------");
}
