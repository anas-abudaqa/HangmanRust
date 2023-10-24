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

fn create_game_word() {



//did this save?



    
/*

const WORD_LENGTH: usize = 6;
    println!("Word length is {}", WORD_LENGTH);
    let mut game_word_array: [String; WORD_LENGTH] = ["_".to_string(); WORD_LENGTH];

    for letter in 1..WORD_LENGTH{
        println!("Letter {} is {}", letter, game_word_array[letter]);
    }


*/    
    
    
   
}

fn read_user_input() -> String{
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Something wrong happened");
    buffer = buffer.trim().to_string();
    buffer
}