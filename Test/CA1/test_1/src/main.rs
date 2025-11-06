use std::io;
fn main() {
    println!("Temperature Converter and Checker");


    let mut input1 = String::new();

    println!("What is the Temperature(in celsius)");
    io::stdin().read_line(&mut input1).expect("Not a valid integer");
    let temp:f32 = input1.trim().parse().expect("Not a valid number");

    let f:f32 = (9.0 / 5.0) * temp + 32.0; //farenheit
    let k:f32 = temp + 273.15; //Kelvin

    println!("The Temperature in celsius is {}",temp);
    println!("The Temperature in farenheit is {}",f);
    println!("The Temperature in Kelvin is {}",k);
    if temp < 0.0 && temp >= -273.0{
        println!("Freezing point");
    }
    else if temp >=0.0 && temp <= 30.0{
        println!("Normal range");
    }
    else if temp > 30.0{
        println!("Hot Temperature");
    }
    else{
        println!("Invalid input (Not a number or exceeds specified limit)");
    }


}
