# SHA1 Cracker

This project is a simple command-line tool written in Rust for cracking SHA-1 hashes using a dictionary (wordlist) attack. It demonstrates basic Rust programming concepts, file I/O, and hash computation.

## Purpose

The main goal of this project is to:

- Illustrate how to implement a brute-force/dictionary attack to recover plaintext passwords from their SHA-1 hashes.
- Provide a practical example of using Rust for security-related tasks.
- Serve as a learning resource for understanding hashing, file handling, and command-line applications in Rust.

## How It Works

### Encode

- The tool takes a plaintext string as input and outputs its SHA-1 hash.

### Decode

1. The tool takes a SHA-1 hash as input.
2. It reads a list of possible passwords from `wordlist.txt`.
3. For each word in the wordlist, it computes the SHA-1 hash and compares it to the target hash.
4. If a match is found, it prints the original password.

## Usage

Please make sure Rust and Cargo installed on your system.

### Encode a String

```bash
cargo run -- encode <PLAINTEXT>
# Example
cargo run -- encode passw0rd

# Output:
# 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
```

### Decode (Crack) a SHA-1 Hash

1. Place your list of possible passwords in `wordlist.txt` (one password per line).
2. Build and run the project with Cargo:

```bash
cargo run -- decode <WORDLIST_PATH> <SHA1_HASH>

# Example
cargo run -- decode wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53

# Output:
# passw0rd
```
