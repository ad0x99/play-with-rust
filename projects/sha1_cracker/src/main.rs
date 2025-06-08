mod utils;

use std::{env, error::Error};
use utils::{SH1_HEX_STRING_LENGTH, sha1_decode, sha1_encode};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args.len() > 3 {
        println!("Usage: ");
        println!("* sha1_cracker - encode your password: <password>");
        println!("* sha1_cracker - decode your password: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    // Encode a password
    if args.len() == 2 {
        let password_to_encode = args[1].trim();
        let encoded_password = sha1_encode(&password_to_encode)?;
        println!("Your password hash: {}", encoded_password);
        return Ok(());
    }

    // Decode a hash to find original password
    if args.len() == 3 {
        let hash_to_crack = args[2].trim();
        if hash_to_crack.len() != SH1_HEX_STRING_LENGTH {
            return Err("sha1 hash is not valid".into());
        }

        let wordlist_path = &args[1];
        sha1_decode(wordlist_path, hash_to_crack)?;
        return Ok(());
    }

    Ok(())
}
