struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand:String::from("HP"),
        price:650000,
    };
    
    let ibm = Laptop {
        brand:String::from("IBM"),
        price:755000,
    };
    
    let toshiba = Laptop {
        brand:String::from("Toshiba"),
        price:550000,
    };
    
    let dell = Laptop {
        brand:String::from("Dell"),
        price:850000,
    };

    
    let amount_purchased = 3;

    
    let hp_cost = hp.calculate_cost(amount_purchased);
    let ibm_cost = ibm.calculate_cost(amount_purchased);
    let toshiba_cost = toshiba.calculate_cost(amount_purchased);
    let dell_cost = dell.calculate_cost(amount_purchased);


    println!("--- Purchase Of 3 Laptop Brands ---");
    println!("Cost for 3 {} laptops: ₦{}", hp.brand, hp_cost);
    println!("Cost for 3 {} laptops: ₦{}", ibm.brand, ibm_cost);
    println!("Cost for 3 {} laptops: ₦{}", toshiba.brand, toshiba_cost);
    println!("Cost for 3 {} laptops: ₦{}", dell.brand, dell_cost);
    println!("-----------------------------------");
}
