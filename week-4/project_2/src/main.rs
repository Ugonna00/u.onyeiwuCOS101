use std::io;

fn main() {
    println!("Incentive calculator");
    println!("\nExperience level(if Experienced write 'experienced', if not, write 'not experienced'):");
    
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    let experience = experience.trim().to_lowercase();

    let mut input2 = String::new();
    println!("\nEnter your age:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    println!("\nWhat is your Experience level?: {}",experience);
    println!("\nWhat is your age: {}", age);
    
    if experience == "experienced" && age >= 40 
    {
            println!("Your Incentive is 1,560,000 per month");
        }
        else if experience == "experienced" && age >= 30 && age < 40
        {
            println!("Your Incentive is i,480,000 per month");
        }
        else if experience == "experienced" && age < 28
    {
        println!("Your Incentive is 1,300,000 per month");
    }
    else if experience == "not experienced"
    {
        println!("Your Incentive is 100,000 per month");
    } 
    else
    {
        println!("Input does not match the Incentive rules.")
    }
    
}
