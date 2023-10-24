use std::io;


fn main() {
    println!("Welcome to Hangman");

    let mut guess = String::new();
    let chosen_word = String::from("Impression");
    //let mut WORD_LENGTH: i32 = chosen_word.chars().count();
    let tries = 6; 

    
    create_game_word();
    println!("Guess a number!");
    for number in (1..tries).rev() {
        guess = read_user_input();
        println!("You guessed {}", guess);
        print!("Keep guessing! ");
        println!("Tries left {}", number);
    }

}

struct Game{
    game_difficulty: String
    game_word: String,




};


fn read_user_input() -> String{
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Something wrong happened");
    buffer = buffer.trim().to_string();
    buffer
}