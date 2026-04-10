use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    surname: String,
    username: String,
    faculty_email: String,
    general_email: String,
}

fn main() {
    use std::io;
    //lesson 3 start topic: strings
    //lesson 5 start topic: vectors, random function
    let mut data = Vec::new();
    let mut user = Vec::new();
    let mut rnd = rand::rng();
    let mut loop_number = String::new();
    io::stdin().read_line(&mut loop_number).expect("Failed to read line");
    let loop_number = i32::from_str_radix(&loop_number.trim(), 10).unwrap();
    for _ in 0..loop_number {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut surname = String::new();
    io::stdin().read_line(&mut surname).expect("Failed to read line");
    
    let name = name.trim();
    let surname = surname.trim();
    let firstpart;
    let secondpart;
    //lesson 4 start topic: controlflow

    if name.len() < 3{
        firstpart = name.get(0..name.len()).unwrap().to_string();
    }
    else{
        firstpart = name.get(0..3).unwrap().to_string();
    }

    if surname.len() < 5{
        secondpart = surname.get(0..surname.len()).unwrap().to_string();
    }
    else{
        secondpart = surname.get(0..5).unwrap().to_string();
    }
    //lesson 4 end topic: controlflow

    let mut username = secondpart.to_lowercase() + &firstpart.to_lowercase();
    //control if there exist any same username; use random function to generate random number and add it to the username
    if data.contains(&username){
        let rnd_number = rnd.random_range(1..100);
        username = username + &rnd_number.to_string();
        //it's possible to use counter for this situation, but it's not random. I prefer random because why not?
    }

    println!("{}", username);
    data.push(username.clone());

    let faculty_email = username.clone() + "@fit.cvut.cz";
    let general_email = username.clone() + "@cvut.cz";

    println!("{}", faculty_email);
    println!("{}", general_email);
    //lesson 3 end 
    user.push(User {
        name: name.to_string(),
        surname: surname.to_string(),
        username: username.to_string(),
        faculty_email: faculty_email.to_string(),
        general_email: general_email.to_string(),
    });
}
println!("{:?}", data); //print all used usernames
let file = File::create("users.json").expect("Failed to create file");
serde_json::to_writer(file, &user).expect("Failed to write to file");
//lesson 5 end topic: vectors, random function
}

