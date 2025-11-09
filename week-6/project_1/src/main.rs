use std::io;
 fn main() {
 println!("\nUse codes P, F, A, E and W to make your order");
 println!(" p = Poundo Yam/Edinkaiki Soup - N3,200
            f = Fried rice & Chicken - N3,000
            a = Amamla & Ewedu Soup- N2,500
            e - Eba & Egusi Soup - N2,000
            w = White Rice & Stew - N2,500");
 let mut input1 = String::new();
 let mut input2 = String::new();

 println!("What do you want?");
 io::stdin().read_line(&mut input1).expect("Not a valid input");
 let food = input1.trim().to_lowercase();

 println!("How many do you want?");
 io::stdin().read_line(&mut input2).expect("Not a valid input");
 let qty:f32 = input2.trim().parse().expect("Not a valid number");

 let amountp:f32 = 3200.0 * qty;
 let amountf:f32 = 3000.0 * qty;
 let amounta:f32 = 2500.0 * qty;
 let amounte:f32 = 2000.0 * qty;
 let amountw:f32 = 2500.0 * qty;

 let damountp:f32 = 0.05 * amountp;
 let damountf:f32 = 0.05 * amountf;
 let damounta:f32 = 0.05 * amounta;
 let damounte:f32 = 0.05 * amounte;
 let damountw:f32 = 0.05 * amountw;

 let tamountp:f32 = amountp - damountp;
 let tamountf:f32 = amountf - damountf;
 let tamounta:f32 = amounta - damounta;
 let tamounte:f32 = amounte - damounte;
 let tamountw:f32 = amountw - damountw;

 
 if food == "p" && amountp >= 10000.0{
    println!("Your amount after discount is {}",tamountp);
 }
  else if food == "p" && amountp < 10000.0{
    println!("Your amount to pay is {}",amountp);
 }
 else if food == "f" && amountf >= 10000.0{
    println!("Your amount after discount is {}",tamountf);
 }
 else if food == "f" && amountf < 10000.0{
    println!("Your amount to pay is {}",amountf);
 }
 else if food == "a" && amounta >= 10000.0{
    println!("Your amount after discount is {}",tamounta);
 }
 else if food == "a" && amounta < 10000.0{
    println!("Your amount to pay is {}",amounta);
 }
 else if food == "e" && amounte >= 10000.0{
    println!("Your amount after discount is {}",tamounte);
 }
 else if food == "e" && amounte < 10000.0{
    println!("Your amount to pay is {}",amounte);
 }
 else if food == "w" && amountw >= 10000.0{
    println!("Your amount after discount is {}",tamountw);
 }
 else if food == "w" && amountw < 10000.0{
    println!("Your amount to pay is {}",amountw);
 }
 else{
    println!("Invalid input");
 }
}
