use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    surname: String,
    username: String,
    faculty_email: String,
    general_email: String,
} //creating referance for how user should be saved

struct Input {
    name: String,
    surname: String,
} //inputs structure

fn input() -> Input {
    let mut name = String::new();
    let mut surname = String::new();

    println!("Enter name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line"); //get name of user

    println!("Enter surname:");
    io::stdin()
        .read_line(&mut surname)
        .expect("Failed to read line"); // get surname of user

    Input {
        name: name.trim().to_string(),
        surname: surname.trim().to_string(),
    }
}

//step 2 -> username_creator
fn username_creator(data: &mut Vec<String>) -> User {
    let input = input();

    //if len(user_name) is less than 3 use for username just len(user_name) but if it's >3 just use first 3 character of name    
    let secondpart = if input.name.len() < 3 {
        input.name.clone() 
    } else {
        input.name[0..3].to_string()
    };
    //if len(user_surname) is less than 5 use for username just len(user_surname) but if it's >5 just use first 5 character of surname    
    let firstpart = if input.surname.len() < 5 {
        input.surname.clone()
    } else {
        input.surname[0..5].to_string()
    };

    //marge with surname+name's selected parts
    let base_username = (firstpart + &secondpart).to_lowercase();
    let mut username = base_username.clone();
    let mut counter = 1;

    //if there have any same username, use counter for add number at the end of username
    while data.contains(&username) {
        username = format!("{}{}", base_username, counter);
        counter += 1;
    }

    data.push(username.clone());
    //get faculty_email and general_email for save as json
    User {
        name: input.name,
        surname: input.surname,
        username: username.clone(),
        faculty_email: format!("{}@fit.cvut.cz", username),
        general_email: format!("{}@cvut.cz", username),
    }
}

fn main() {
    let mut data: Vec<String> = Vec::new();
    //use loop for user want to add 1+ user to json
    loop {
        let user = username_creator(&mut data);
        println!("{:?}", user);
        //create users.json on base folder
        let file = File::create("users.json").expect("Failed to create file");
        serde_json::to_writer_pretty(file, &user).expect("Failed to write to file");
        //ask if user want to continue
        let mut answer = String::new();
        println!("Do you want to create another user? (yes/no)");
        
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        //if answer is not yes, just end the program
        if answer.trim().to_lowercase() != "yes" {
            break;
        }
    }
}