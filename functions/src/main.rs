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
}

struct Input {
    name: String,
    surname: String,
}

fn input() -> Input {
    let mut name = String::new();
    let mut surname = String::new();

    println!("Enter name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter surname:");
    io::stdin()
        .read_line(&mut surname)
        .expect("Failed to read line");

    Input {
        name: name.trim().to_string(),
        surname: surname.trim().to_string(),
    }
}

fn username_creator(data: &mut Vec<String>) -> User {
    let input = input();

    let secondpart = if input.name.len() < 3 {
        input.name.clone()
    } else {
        input.name[0..3].to_string()
    };

    let firstpart = if input.surname.len() < 5 {
        input.surname.clone()
    } else {
        input.surname[0..5].to_string()
    };

    let base_username = (firstpart + &secondpart).to_lowercase();
    let mut username = base_username.clone();
    let mut counter = 1;

    while data.contains(&username) {
        username = format!("{}{}", base_username, counter);
        counter += 1;
    }

    data.push(username.clone());

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

    loop {
        let user = username_creator(&mut data);
        println!("{:?}", user);

        let file = File::create("users.json").expect("Failed to create file");
        serde_json::to_writer_pretty(file, &user).expect("Failed to write to file");

        let mut answer = String::new();
        println!("Do you want to create another user? (yes/no)");

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim().to_lowercase() != "yes" {
            break;
        }
    }
}