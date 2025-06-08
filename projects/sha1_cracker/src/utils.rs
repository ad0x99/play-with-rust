use sha1::Digest;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub const SH1_HEX_STRING_LENGTH: usize = 40;

pub fn sha1_encode(password: &str) -> Result<String, Box<dyn Error>> {
    // Hash the password with SHA-1
    let hash = sha1::Sha1::digest(password.as_bytes());

    // Encode the hash as hex
    let encoded_hash = hex::encode(hash);
    Ok(encoded_hash)
}

pub fn sha1_decode(wordlist_path: &String, hash: &str) -> Result<(), Box<dyn Error>> {
    // Open wordlist find using given path
    let wordlist_file = File::open(&wordlist_path)?;
    // Read line by line to find possible password
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        // Propagate errors and extract data when reading each line
        let line = line?;
        // Trim whitespace from each line
        let common_password = line.trim();
        // Hash the password candidate with SHA-1 and encode the hash as a hex string
        let decoded_hash = &hex::encode(sha1::Sha1::digest(common_password.as_bytes()));

        // If a match is found, print the found password
        if hash == decoded_hash {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

    // Otherwise, print a failure message
    println!("Password not found in wordlist :(!");
    Ok(())
}
