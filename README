# User Generator in Rust

This project is a simple Rust program that generates a university-style user account from a person's **name** and **surname**.

The program:
- reads a user's name and surname from standard input,
- creates a username based on those values,
- generates two email addresses,
- stores the result in a `User` struct,
- saves the output into a `users.json` file.

## Features

- Takes user input from the terminal
- Automatically creates a username
- Generates:
  - `@fit.cvut.cz` faculty email
  - `@cvut.cz` general email
- Serializes data into JSON using `serde` and `serde_json`
- Basic duplicate username handling with a random number

## Project Structure

### `User`
This struct stores the final generated user data:
- `name`
- `surname`
- `username`
- `faculty_email`
- `general_email`

### `Input`
This struct is used for temporary input storage:
- `name`
- `surname`

### `input()`
This function:
- reads `name` and `surname` from terminal input,
- trims newline characters,
- returns an `Input` object.

### `username_creator(data: &mut Vec<String>) -> User`
This function:
- gets input from the user,
- creates a username using parts of the surname and name,
- converts the username to lowercase,
- checks if the username already exists,
- adds a random number if needed,
- creates a `User` object with generated emails,
- returns the final user.

## Username Logic

The username is created with this logic:
- take the first **up to 5 characters** from the surname,
- take the first **up to 3 characters** from the name,
- combine them,
- convert the result to lowercase.

Example:
- Name: `Rufat`
- Surname: `Azizzade`
- Username: `azizzruf`

Generated emails:
- `azizzruf@fit.cvut.cz`
- `azizzruf@cvut.cz`

## How to Run

### 1. Install Rust
Make sure Rust is installed on your system.

### 2. Add dependencies
Your `Cargo.toml` should include:

```toml
[dependencies]
rand = "0.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. Run the program

```bash
cargo run
```

### 4. Enter input
After running, type:
- first line: name
- second line: surname

Example:

```text
Rufat
Azizzade
```

## Output

The program prints the generated `User` object and saves it into:

```text
users.json
```

Example JSON output:

```json
{
  "name": "Rufat",
  "surname": "Azizzade",
  "username": "azizzruf",
  "faculty_email": "azizzruf@fit.cvut.cz",
  "general_email": "azizzruf@cvut.cz"
}
```

## Notes

- This is a beginner-friendly Rust project for practicing:
  - structs
  - functions
  - mutable references
  - JSON serialization
  - string handling
- The current duplicate check only works if usernames are actually stored in the `data` vector.
- Using string slicing with Unicode characters may cause issues. A safer version would use `.chars()`.
- Some parts of the code can still be improved for cleaner logic and stronger error handling.

## Possible Improvements

- Store multiple users in a `Vec<User>`
- Save and reload existing users from `users.json`
- Improve duplicate username generation with a loop
- Handle Unicode names safely
- Separate logic into modules
- Add tests

## Learning Goals

This project is useful for learning:
- Rust structs
- ownership and cloning
- input handling with `stdin`
- vectors and references
- file creation
- JSON serialization/deserialization

---

Created for practicing Rust with a real-life style username and email generator.