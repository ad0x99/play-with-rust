# Introduction

A command line tool is called `minigrep` using for searching text in a file

This is used to demonstrate some basic I/O operation concepts, error handling and testing in Rust

# How To Run

Using following command to run the app

```bash
# To find `frog` string in the `poem.txt` file
# with IGNORE_CASE equals to true, it means ignoring case-sensitive
cargo run -- frog poem.txt IGNORE_CASE=1

# To find `frog` string in the `poem.txt` file
# and write the result into `output.txt` file
cargo run -- frog poem.txt > output.txt

# To trigger the error case
# we run the command without parameter
cargo run
```
