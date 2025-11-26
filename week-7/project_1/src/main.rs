use std::io;
use std::f64::consts::PI;
fn trapezium(){
    let mut input = String::new();
    println!("What is the height?");
    io::stdin().read_line(&mut input).expect("Not a valid integer");
    let hgt:f32 = input.trim().parse().expect("Invalid input");

    let mut input1 = String::new();
    println!("What is base a?");
    io::stdin().read_line(&mut input1).expect("Not a valid integer");
    let base1:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("What is base b?");
    io::stdin().read_line(&mut input2).expect("Not a valid integer");
    let base2:f32 = input2.trim().parse().expect("Invalid input");

    let tpz:f32 = 0.5 * (base1 + base2) * hgt;
    println!("The area is {} ",tpz);
}
fn rhombus(){
    let mut diog1 = String::new();
    println!("What is diogonal 1?");
    io::stdin().read_line(&mut diog1).expect("Not a valid integer");
    let diogonal1:f32 = diog1.trim().parse().expect("Invalid input");

    let mut diog2 = String::new();
    println!("What is diogonal 2?");
    io::stdin().read_line(&mut diog2).expect("Not a valid integer");
    let diagonal2:f32 = diog2.trim().parse().expect("Invalid input");

    let rhb:f32 = 0.5 * (diogonal1 * diagonal2);
    println!("The area is {} ",rhb);
}
fn parallelogram(){
    let mut base = String::new();
    println!("What is the base?");
    io::stdin().read_line(&mut base).expect("Not a valid integer");
    let b:f32 = base.trim().parse().expect("Invalid input");

    let mut altitude = String::new();
    println!("What is the altitude?");
    io::stdin().read_line(&mut altitude).expect("Not a valid integer");
    let a:f32 = altitude.trim().parse().expect("Invalid input");

    let pll:f32 = b * a;
    println!("The area is {} ",pll);
}
fn cube(){
    let mut length = String::new();
    println!("What is the length?");
    io::stdin().read_line(&mut length).expect("Not a valid integer");
    let l:f32 = length.trim().parse().expect("Invalid input");


    let cbe:f32 = 6.0 * l.powf(2.0);
    println!("The area is {} ",cbe);
}
fn cylinder(){
    let mut radius = String::new();
    println!("What is the radius?");
    io::stdin().read_line(&mut radius).expect("Not a valid integer");
    let rds:f64 = radius.trim().parse().expect("Invalid input");

    let mut height = String::new();
    println!("What is the height?");
    io::stdin().read_line(&mut height).expect("Not a valid integer");
    let hgt:f64 = height.trim().parse().expect("Invalid input");

    let cld:f64 = PI * (rds.powf(2.0)) * hgt;
    println!("The area is {} ",cld);
}

fn main() {
    println!("Area and volume calculator");
    println!("\nWhat do you want to calculate? Use the codes.");
    println!("Area of a Trapezium- t ");
    println!("Area of a Rhombus- r ");
    println!("Area of a Parallelogram- p ");
    println!("Area of a Cube- c ");
    println!("Volume of a Cylinder- cl ");

    let mut code = String::new();
    println!("\nCODE?");
    io::stdin().read_line(&mut code).expect("Not a valid integer");
    let acode = code.trim().to_lowercase();

    if acode == "t"{
        trapezium();
    }
    else if acode == "r"{
        rhombus();
    }
    else if acode == "p"{
        parallelogram();
    }
    else if acode == "c"{
        cube();
    }
    else if acode == "cl"{
        cylinder();
    }
    else{
        println!("Invalid input");
    }
}
