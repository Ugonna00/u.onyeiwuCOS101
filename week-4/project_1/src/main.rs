use std::io;

fn main() {
    println!("\nQuadratic equation calculator");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nWhat is the value of a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\nWhat is the value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("\nWhat is the value of c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = b.powf(2.0) - 4.0 * (a * c); //discriminant
    let rd:f32 = d.sqrt();
    let root1:f32 = (-b + rd)/ 2.0 * a;
    let root2:f32 = (-b - rd)/ 2.0 * a; 

    if d > 0.0{
        println!("There are two distinct roots: {} and {}",root1,root2);
    }
    else if d == 0.0{
        println!("There is exactly one real root: {}",root1);
    }
    else{
        println!("There are no real roots");
    }
}
