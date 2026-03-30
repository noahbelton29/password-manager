# Simple Password Manager

This is a **very basic password manager** written in Rust. It lets you **store, view, and delete password entries** from a simple text file, for now it is not recommended that you use it.

> **Note:** This is for learning purposes only. Passwords are stored in plain text, do not use it for sensitive accounts.

---

## Features

- **Create a new entry** with a heading (like "Gmail"), email/username, and password.
- **View all saved entries** in a readable format.
- **Delete entries** by heading. If multiple entries share the same heading, you can choose which one to delete.

---

## How It Works

1. Each password entry is stored in a text file (`passwords.txt`) in the format:  
   `Heading:Email:Password`

2. The program reads and writes to this file whenever you add, view, or delete entries.

## Usage

1. Run the program:

```bash
cargo run