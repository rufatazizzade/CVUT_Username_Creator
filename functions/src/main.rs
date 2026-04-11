use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use rand::Rng;
use std::io;
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    surname: String,
    username: String,
    faculty_email: String,
    general_email: String,
} // we struct for saving data in json file basicly we giving reference to code to how code should data look like

struct Input {
    name: String,
    surname: String,
}

fn input() -> Input {
    let mut name = String::new();
    let mut surname = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    io::stdin().read_line(&mut surname).expect("Failed to read line");
    let user = Input {
        name: name.trim().to_string(),
        surname: surname.trim().to_string(),
    };
    user
}

fn username_creator(data: &mut Vec<String>) -> User{
    let input = input();
    let mut username;
    let mut secondpart;
    let mut firstpart;
    let mut rnd = rand::rng();
    if input.name.len() < 3{
        secondpart = input.name.get(0..input.name.len()).unwrap().to_string();
    }
    else{
        secondpart = input.name.get(0..3).unwrap().to_string();
    }
    if input.surname.len() < 5{
        firstpart = input.surname.get(0..input.surname.len()).unwrap().to_string();
    }
    else{
        firstpart = input.surname.get(0..5).unwrap().to_string();
    }
    username = firstpart + &secondpart;
    username = username.to_lowercase();

    if data.contains(&username){
        let rnd_number = rnd.random_range(1..100);
        username = username + &rnd_number.to_string();
    }
    let user = User {
        name: input.name,
        surname: input.surname,
        username: username.clone(),
        faculty_email: username.clone() + "@fit.cvut.cz",
        general_email: username.clone() + "@cvut.cz",
    };
    user
}

fn main(){
    let mut data: Vec<String> = Vec::new();
    let mut user = User {
        name: String::new(),
        surname: String::new(),
        username: String::new(),
        faculty_email: String::new(),
        general_email: String::new(),
    };
    user = username_creator(&mut data);
    println!("{:?}", user);
    let file = File::create("users.json").expect("Failed to create file");
    serde_json::to_writer(file, &user).expect("Failed to write to file");
}