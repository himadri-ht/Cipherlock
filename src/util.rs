use ::dialoguer::Input;
use dirs;
use rpassword;
/**
 * util.rs
 * Utility functions to avoid code reuse.
 */
use std::fs::{create_dir_all, read, remove_file, File};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::{db, DbFile, CONF_FILE_EXT, CONF_NAME, DIR_NAME};

pub fn create_file_with_data(path: &PathBuf, data: &String) {
    let prefix = path.parent().unwrap();
    create_dir_all(prefix).unwrap();

    // display is a helper struct for safely printing paths
    let display = path.display();

    // open a file
    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => file,
    };

    // write to file
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote data to {}", display),
    }
}

pub fn create_empty_file(path: &PathBuf) -> File {
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("Cannot create file at {}: {}", display, why),
        Ok(file) => file,
    };

    file
}

pub fn write_bytes_to_file(mut file: File, data: &Vec<u8>) {
    match file.write_all(data) {
        Err(why) => panic!("Cannot write data to file: {}", why),
        Ok(_) => (),
    }
}

pub fn read_as_bytes(path: &PathBuf) -> Vec<u8> {
    let display = path.display();

    let bytes = match read(&path) {
        Err(why) => panic!("Cannot read {}: {}", why, display),
        Ok(bytes) => bytes,
    };

    bytes
}

pub fn create_dir(dir_path: &PathBuf) {
    match create_dir_all(&dir_path) {
        Err(why) => panic!("Could not create directories {:?}: {}", &dir_path, why),
        Ok(_) => println!("Directories created successfully : {:?}.", dir_path),
    };
}

pub fn get_homedir() -> PathBuf {
    let homedir = dirs::home_dir().expect("Could not get home directory");

    homedir
}

// parses CipherLock config to get the db file location
pub fn get_db_location() -> PathBuf {
    let mut conf_path = PathBuf::new();

    let home_dir = get_homedir();

    conf_path.push(home_dir);
    conf_path.push(DIR_NAME);
    conf_path.push(CONF_NAME);
    conf_path.set_extension(CONF_FILE_EXT);

    // make pathbuf printable.
    let display = conf_path.display();

    // parse the configuration and get the db location
    let mut s = String::new();
    let mut file = match File::open(&conf_path) {
        Err(why) => panic!("Could not open : {} {}", display, why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read as string: {} {}", display, why),
        Ok(file) => file,
    };

    let d: DbFile = serde_json::from_str(&s).unwrap();

    let mut db_location: PathBuf = PathBuf::new();
    db_location.push(d.path);
    db_location.push(d.name);

    db_location
}

pub fn get_password(prompt: &String) -> String {
    let password =
        rpassword::prompt_password(prompt).expect("An error occured while getting password input");

    password
}

pub fn remove_file_from_path(path: &PathBuf) {
    remove_file(path).expect("Failed to remove the file.");
}

pub fn get_input(prompt: &String) -> String {
    let input: String = Input::new().with_prompt(prompt).interact_text().unwrap();

    input
}

pub fn get_pass_len(prompt: &String) -> usize {
    let input: usize = Input::new().with_prompt(prompt).interact_text().unwrap();

    input
}

pub fn update_encrypted_database_entries(
    db_location: &PathBuf,
    master_password: &String,
    tmp_path: &PathBuf,
) {
    //remove previous database file
    remove_file_from_path(db_location);

    let f = create_empty_file(db_location);

    let encrypted_tmp_file = db::encrypt_db(&tmp_path, &master_password);

    let encrypted_data = read_as_bytes(&encrypted_tmp_file);

    write_bytes_to_file(f, &encrypted_data);

    remove_file_from_path(&tmp_path);
}
