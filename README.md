# Password Generator

This Rust project generates a list of passwords based on user input. It utilizes various input types such as keywords, names, passwords, numbers, symbols, and birthdays to generate a comprehensive list of potential passwords.

## Features

 - *Input Types*: Supports keywords, names, passwords, numbers, symbols, and birthdays (in the format mm-dd-yyyy).
 - *Variations*: Generates multiple variations of each input by appending or combining with suffixes, years, days, and months.
 - *Output*: Outputs generated passwords to a file named `passwords.txt`.

## Usage 

### Prerequisites

 - Ensure you have Rust installed on your system. If not, you can install it from rust-lang.org.

### Running the Program

1. Clone the repository or download the `main.rs` file to your local machine.
2. Navigate to the directory containing `main.rs` using the terminal.
3. Run the program using Cargo, Rust's package manager and build system:
```
cargo run 
```
or
```
rustc main.rs -o a
./a
```
4. Follow the prompts in the terminal to input keywords, names, passwords, numbers, symbols, or birthdays. To generate the password list, input `generate`.
5. Once the generation process is complete, the program will create a file named `passwords.txt` in the current directory, containing the generated passwords.
