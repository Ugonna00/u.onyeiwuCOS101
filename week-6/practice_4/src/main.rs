fn main() {
    let fullname = "Chibudum John Umeh";
    let departmenet = "Computer Sceince";
    let uni = "Pan-Atlantic University";


    let mut school = "School of science".to_string();
    //push string
    school.push_str(" and Technology");

    println!("My name is {}", fullname);
    //check length
    println!("The length of my full name is: {}",fullname.len());
    println!("I am a student of {} departmenet", departmenet);
    println!("{}",school);
    println!("{}",uni);
}
