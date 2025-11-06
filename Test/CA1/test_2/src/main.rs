use std::io;
fn main() {
    loop{
    println!("Employee Payroll Calculator");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What is your name");
    io::stdin().read_line(&mut input1).expect("Not a valid integer");
    let name = input1.trim().to_lowercase();

    println!("How many hours have you worked?");
    io::stdin().read_line(&mut input2).expect("Not a valid integer");
    let hrs:f32 = input2.trim().parse().expect("Not a valid number");

    let salary:f32 = 3000.0 * hrs;
    let xhrs:f32 = 4500.0 * hrs;
    let pat:f32 = salary - 2000.0; //net pay after tax

    println!("Description:");
    println!("\nName: {}",name);
    println!("Your gross salary is {}",salary);
    println!("Your hours worked is {}",hrs);
    if salary > 100_000.0{
        println!("Hello, {},Your net salary after tax is {}", name,pat);
    }
    if hrs > 40.0{
        println!("Hello, {},Due to extra hours, your salary is {}",name,xhrs);
    }

println!("Do you want to perform another calculation? (yes/no)");
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

