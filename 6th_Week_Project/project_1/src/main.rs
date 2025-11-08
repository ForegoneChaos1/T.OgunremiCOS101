use std::io;

fn main() {
    println!("|=============================MENU==============================|");
    println!("|     Code      |             Name               |   Price      |");
    println!("|---------------------------------------------------------------|");
    println!("|       P       |   Pounded Yam/ Edinkaiko Soup  |   3200       |");
    println!("|---------------------------------------------------------------|");
    println!("|       F       |      Fried Rice & Chicken      |   3000       |");
    println!("|---------------------------------------------------------------|");
    println!("|       A       |      Amala & Ewedu Soup        |   2500       |");
    println!("|---------------------------------------------------------------|");
    println!("|       E       |       Eba & Egusi Soup         |   2000       |");
    println!("|---------------------------------------------------------------|");
    println!("|       W       |       White Rice & Stew        |   2500       |");
    println!("|---------------------------------------------------------------|");

    println!("Enter food type (P, F, A, E, W): ");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read input");
    let food_code = food_code.trim().to_uppercase();

    
    println!("Enter quantity: ");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty: i32 = qty.trim().parse().expect("Please enter a number");

    // Determine price per item
    let price = match food_code.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food code entered!");
            return;
        }
    };

    let mut total = price * qty;

    // Apply discount if applicable
    if total > 10_000 {
        let discount = (total as f64) * 0.05;
        total = (total as f64 - discount) as i32;
        println!("A 5% discount has been applied!");
    }

    println!("Total : ₦{}", total);
    println!("Thank you for ordering!");
}