use std::{
    collections::HashSet,
    fs::File,
    io::{stdin, Write, stdout},
};

fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut result_string: String = String::new();
            let mut is_first = true;
            for c in word.chars(){
                if is_first && c.is_alphabetic(){
                    result_string = format!("{}{}", result_string, c.to_uppercase());  
                    is_first = false;
                    continue;
                }
                result_string = format!("{}{}", result_string, c);  
            }
            result_string 
        })
        .collect::<Vec<String>>()
        .join(" ")
}

struct List {
    items: Vec<String>,
    set: HashSet<String>,
}

impl List {
    fn new() -> List {
        List {
            items: Vec::new(),
            set: HashSet::new(),
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn append(&mut self, item: &str, front: bool) {
        let variations = vec![
            item.to_string(),
            title_case(item),
            item.to_lowercase(),
            item.to_uppercase(),
        ];

        for variation in variations {
            if !self.set.contains(&variation) {
                self.set.insert(variation.clone());
                if front {
                    self.items.insert(0, variation);
                } else {
                    self.items.push(variation);
                }
            }
        }
    }
}

struct PassGen {
    words: Vec<String>,
    b_days: Vec<String>,
    is_alive: bool,
    password_list: List,
    suffix: Vec<String>,
}

impl PassGen {
    fn new() -> PassGen {
        PassGen {
            words: Vec::new(),
            b_days: Vec::new(),
            is_alive: true,
            password_list: List::new(),
            suffix: (0..124).map(|c| c.to_string()).collect(),
        }
    }

    fn get_input(&mut self) {
        while self.is_alive {
            println!("Enter a keyword, name, password, number, symbol, or birthday(mm-dd-yyyy)");
            println!("To generate a password list enter generate");
            print!("\n$> ");
            stdout().flush().unwrap();
            let mut user_input = String::new();
            if stdin().read_line(&mut user_input).is_err() {
                self.is_alive = false;
            }

            if !self.is_alive || user_input.trim().is_empty() {
                continue;
            }

            if user_input.trim().eq_ignore_ascii_case("generate") {
                self.generate();
                self.is_alive = false;
            } else {
                self.append_data(user_input.trim().to_string());
            }
        }
        println!("\n");
    }

    fn append_data(&mut self, data: String) {
        if data.split('-').count() == 3 {
            if !self.b_days.contains(&data) {
                self.b_days.push(data);
            }
        } else if data.chars().all(|c| c.is_digit(10)) {
            if !self.suffix.contains(&data) {
                self.suffix.insert(0, data.clone());
            }
            self.password_list.append(&data, true);
        } else if data.chars().all(|c| c.is_alphanumeric()) {
            if !self.words.contains(&data.to_lowercase()) {
                self.words.push(data);
            }
        } else {
            self.password_list.append(&data, true);
        }
    }

    fn generate(&mut self) {
        println!("Generating a list, this might take a while");
        for suffix in &self.suffix {
            for word in &self.words {
                self.password_list.append(&word, false);
                self.password_list.append(&format!("{}{}", word, suffix), false);
                self.password_list.append(&format!("{}{}", suffix, word), false);
                self.password_list.append(&format!("{}{}{}", suffix, word, suffix), false);

                for bday in &self.b_days {
                    let parts: Vec<&str> = bday.split('-').collect();
                    let day = parts[1];
                    let month = parts[0];
                    let year = parts[2];
                    let plain_bday = bday.replace("-", "");

                    self.password_list.append(&plain_bday, false);
                    self.password_list
                        .append(&format!("{}{}", word, year), false);
                    self.password_list.append(
                        &format!("{}{}", word, year.chars().skip(2).collect::<String>()),
                        false,
                    );
                    self.password_list
                        .append(&format!("{}{}", word, plain_bday), false);

                    self.password_list
                        .append(&format!("{}{}", day, word), false);
                    self.password_list
                        .append(&format!("{}{}", day.chars().last().unwrap(), word), false);

                    self.password_list
                        .append(&format!("{}{}", year, word), false);
                    self.password_list.append(
                        &format!("{}{}", year.chars().skip(2).collect::<String>(), word),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}", month, word), false);
                    self.password_list
                        .append(&format!("{}{}", month.chars().last().unwrap(), word), false);

                    self.password_list
                        .append(&format!("{}{}{}", month, day, word), false);
                    self.password_list.append(
                        &format!("{}{}{}", month.chars().last().unwrap(), day, word),
                        false,
                    );
                    self.password_list.append(
                        &format!("{}{}{}", month, day.chars().last().unwrap(), word),
                        false,
                    );
                    self.password_list.append(
                        &format!(
                            "{}{}{}",
                            month.chars().last().unwrap(),
                            day.chars().last().unwrap(),
                            word
                        ),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}", day, month, word), false);
                    self.password_list.append(
                        &format!("{}{}{}", day.chars().last().unwrap(), month, word),
                        false,
                    );
                    self.password_list.append(
                        &format!("{}{}{}", day, month.chars().last().unwrap(), word),
                        false,
                    );
                    self.password_list.append(
                        &format!(
                            "{}{}{}",
                            day.chars().last().unwrap(),
                            month.chars().last().unwrap(),
                            word
                        ),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}{}", month, day, word, year), false);
                    self.password_list
                        .append(&format!("{}{}{}{}", month, day, word, &year[2..]), false);
                    self.password_list.append(
                        &format!("{}{}{}{}", month.chars().last().unwrap(), day, word, year),
                        false,
                    );
                    self.password_list.append(
                        &format!(
                            "{}{}{}{}",
                            month.chars().last().unwrap(),
                            day,
                            word,
                            &year[2..]
                        ),
                        false,
                    );

                    self.password_list.append(
                        &format!("{}{}{}{}", month, day.chars().last().unwrap(), word, &year),
                        false,
                    );
                    self.password_list.append(
                        &format!(
                            "{}{}{}{}",
                            month,
                            day.chars().last().unwrap(),
                            word,
                            &year[2..]
                        ),
                        false,
                    );

                    self.password_list.append(
                        &format!(
                            "{}{}{}{}",
                            month.chars().last().unwrap(),
                            day.chars().last().unwrap(),
                            word,
                            year
                        ),
                        false,
                    );
                    self.password_list.append(
                        &format!(
                            "{}{}{}{}",
                            month.chars().last().unwrap(),
                            day.chars().last().unwrap(),
                            word,
                            &year[2..]
                        ),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}", month, word, suffix), false);
                    self.password_list.append(
                        &format!("{}{}{}", month.chars().last().unwrap(), word, suffix),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}", day, word, suffix), false);
                    self.password_list.append(
                        &format!("{}{}{}", day.chars().last().unwrap(), word, suffix),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}", suffix, word, month), false);
                    self.password_list.append(
                        &format!("{}{}{}", suffix, word, month.chars().last().unwrap()),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}", suffix, word, day), false);
                    self.password_list.append(
                        &format!("{}{}{}", suffix, word, day.chars().last().unwrap()),
                        false,
                    );

                    self.password_list
                        .append(&format!("{}{}{}", suffix, word, year), false);
                    self.password_list
                        .append(&format!("{}{}{}", suffix, word, &year[2..]), false);
                 }
            }
        }

        let mut file = File::create("passwords.txt").expect("Unable to create file");
        println!("{}", self.password_list.len());
        for password in &self.password_list.items {
            writeln!(file, "{}", password).expect("Unable to write data");
        }
    }
}

fn main() {
    let mut passgen = PassGen::new();
    passgen.get_input();
}

