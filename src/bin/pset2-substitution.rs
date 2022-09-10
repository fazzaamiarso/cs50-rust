use dialoguer::Input;
use std::env::args;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz"
        .char_indices()
        .collect::<Vec<(usize, char)>>();

    if args().len() != 2 {
        println!("Usage: command <KEY>");
        return;
    }

    let key = args().nth(1).unwrap();
    if key.chars().count() != 26 {
        println!("Key must contains 26 characters!");
        return;
    }

    let plain_text = Input::<String>::new()
        .with_prompt("plain text")
        .interact_text()
        .unwrap();

    let cipher_vec = plain_text
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let idx = alphabet
                    .iter()
                    .position(|&x| x.1 == c.to_ascii_lowercase())
                    .unwrap();
                let el = key.chars().collect::<Vec<char>>()[idx];
                if c.is_uppercase() {
                    return el.to_ascii_uppercase();
                } else {
                    return el.to_ascii_lowercase();
                }
            }
            return c;
        })
        .collect::<Vec<char>>();

    let cipher_text: String = cipher_vec.iter().collect();
    println!("cipher text: {}", cipher_text);
}
