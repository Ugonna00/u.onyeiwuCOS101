use std::io;
fn main() {
    loop{
    println!("Bookshop Order System");
    println!("\nWhat would you like?");
    println!("Book title- Rust for beginners (15,000) CODE: R");
    println!("Book title- AI basics (12,500) CODE: A");
    println!("Book title- Data Structures in Rust (20,000) CODE: D");
    println!("Book title- Networking essentials (18,000) CODE: N");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Using the book code, what book do you want?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let code = input1.trim().to_lowercase();

    println!("How many do you want?");
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let qty:f32 = input2.trim().parse().expect("Not a valid integer");

    let tar:f32 = 15000.0 * qty; //Total amount for R
    let taa:f32 = 12500.0 * qty; //Toatl amount for A
    let tad:f32 = 20000.0 * qty; //Toatl amount for D
    let tan:f32 = 18000.0 * qty; //Toatl amount for N

    let dtar:f32 = 0.10 * tar; //Discounted Total amount for R
    let dtaa:f32 = 0.10 * taa; //Discounted Total amount for A
    let dtad:f32 = 0.10 * tad; //Discounted Total amount for D
    let dtan:f32 = 0.10 * tan; //Discounted Total amount for N

    let ftar:f32 = tar - dtar; //Final Total amount after discount for R
    let ftaa:f32 = taa - dtaa; //Final Total amount after discount for A
    let ftad:f32 = tad - dtad; //Final Total amount after discount for D
    let ftan:f32 = tan - dtan; //Final Total amount after discount for N

    if code == "r" && qty <= 3.0 {
        println!("Total amount = {}",tar);
    }
    else if code == "r" && qty > 3.0{
        println!("Your amount afer discount is {}",ftar);
    }
    else if code == "a" && qty <= 3.0{
        println!("Total amount = {}",taa);
    }
    else if code == "a" && qty > 3.0{
        println!("Your amount afer discount is {}",ftaa);
    }
    else if code == "d" && qty <= 3.0{
        println!("Total amount = {}",tad);
    }
    else if code == "d" && qty > 3.0{
        println!("Your amount afer discount is {}",ftad);
    }
    else if code == "n" && qty <= 3.0{
        println!("Total amount = {}",tan);
    }
    else if code == "n" && qty > 3.0{
        println!("Your amount afer discount is {}",ftan);
    }
    else{
        println!("Invalid input");
    }

    

println!("Do you want to buy another book? (yes/no)");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid integer");
    let answer = input3.trim().to_lowercase();

    if answer == "no"{
        break;
    }
    else if answer == "yes"{
        println!("Restarting code");
        main();
    }
}

    
}
