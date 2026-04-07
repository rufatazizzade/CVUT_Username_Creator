use std::io;
fn main() {
    //lesson 3 start topic: strings
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut surname = String::new();
    io::stdin().read_line(&mut surname).expect("Failed to read line");

    let username = surname.get(0..5).unwrap().to_string() + name.get(0..3).unwrap();
    let username = username.to_lowercase();

    println!("{}", username);

    let faculty_email = username.clone() + "@fit.cvut.cz";
    let general_email = username.clone() + "@cvut.cz";

    println!("{}", faculty_email);
    println!("{}", general_email);
    //lesson 3 end 


}
