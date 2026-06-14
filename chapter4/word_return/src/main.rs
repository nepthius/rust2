use std::io;

fn main() {
    let mut user_word = String::new();

    io::stdin()
        .read_line(&mut user_word)
        .expect("ruh roh");

    let word = return_word(&user_word);

    if word.is_some(){
        println!("Your word: {word:?}");
    }
    
}

fn return_word(s: &String) -> Option<&str>{
    Some(s.split(" ").next()?.trim())
}
