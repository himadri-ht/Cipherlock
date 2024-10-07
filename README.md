# Cipherlock

## About the project

A lightweight Rust tool for securely storing and retrieving passwords from an offline database. It checks for leaked passwords to prevent reuse attacks and requires no external services. Designed for simplicity, it offers a minimal, easy-to-use command-line interface.

## Features

- **Add Passwords**: Store passwords securely in an offline database, with each entry associated with a domain.
- **Update Passwords**: Modify stored passwords for any domain at any time.
- **List Records**: Display all stored records within the database.
- **Retrieve & Copy**: Fetch any record and copy the associated username and password to the clipboard.
- **Delete Records**: Remove any stored record from the database.
- **Leak Check**: Verify if a password associated with a domain has been previously leaked to prevent reuse attacks.
- **Database Protection**: The database is secured with a username and password, requiring authentication for any modifications.

## Installation

Clone this repository.

```sh
git clone https://github.com/himadri-ht/Cipherlock.git
```

Run the application.

```sh
cd Cipherlock
cargo install --path
```

> Note: Install pkg-config and libssl-dev packages for ubuntu.

## Instructions

- Initially, the database must be initialized using this command.

```sh
cipherlock init --db-path ~
# Initializes the database in the home directory 
```

- Run the help command to list all the commands.

```sh
cipherlock -h
# Opens the help menu
```

```sh
cipherlock 0.9.7

USAGE:
    cipherlock [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -d, --debug      
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    clip      Copy password to clipboard by domain
    delete    Delete a key value pair from database
    get       Copy username and then password to clipboard by domain
    help      Print this message or the help of the given subcommand(s)
    init      Initialize Cipherlock
    insert    Insert a user password pair associated with a domain to database
    leaked    Check if a password associated with your domain is leaked
    list      Lists every record in the database
    update    Update a record from database
```
