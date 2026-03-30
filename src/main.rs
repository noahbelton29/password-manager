mod entry;
mod storage;

use entry::Entry;
use storage::*;

use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_entry() {
    let heading = read_input("Heading: ");
    let email = read_input("Email: ");
    let password = read_input("Password: ");
    let confirm = read_input("Confirm: ");

    if password != confirm {
        eprintln!("Passwords don't match!");
        return;
    }

    let entry = Entry { heading, email, password };
    save_entry(&entry);

    println!("Saved!");
}

fn delete_entry() {
    let target = read_input("Enter the heading to delete: ");
    let entries = load_entries();
    
    let matches: Vec<(usize, &Entry)> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.heading == target)
        .collect();

    if matches.is_empty() {
        println!("No entries found with that heading.");
        return;
    }

    let index_to_delete = if matches.len() == 1 {
        matches[0].0
    } else {
        println!("Multiple entries found:");
        for (i, (_, e)) in matches.iter().enumerate() {
            println!("{}. Email: {}", i + 1, e.email);
        }
        let choice = read_input("Pick a number: ");
        match choice.parse::<usize>() {
            Ok(n) if n > 0 && n <= matches.len() => matches[n - 1].0,
            _ => {
                eprintln!("Invalid choice");
                return;
            }
        }
    };

    delete_entry_by_index(index_to_delete);
    println!("Deleted.");
}

fn view_entries() {
    let entries = load_entries();
    for e in entries {
        println!("{}", e.heading);
        println!("  Email: {}", e.email);
        println!("  Password: {}", e.password);
    }
}

fn main() {
    println!("Welcome to the Password Manager");

    loop {
        println!("Password Manager Menu:");
        println!("  1. Create a new entry");
        println!("  2. Delete an entry");
        println!("  3. View entries");
        println!("  q. Quit");

        let choice = read_input("Pick an option: ");

        match choice.as_str() {
            "1" => create_entry(),
            "2" => delete_entry(),
            "3" => view_entries(),
            "q" => break,
            _ => eprintln!("Invalid option"),
        }
    }
}