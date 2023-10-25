use std::io;


fn main() {
    println!("Welcome to Hangman");

    //let mut guess = String::new();
    //let mut WORD_LENGTH: i32 = chosen_word.chars().count();
    let tries = 6;
    let game_word = String::from("Penis");

    let mut hangman = Game{
        game_word_array: Vec::new(),
        used_letters: Vec::new()
    };
    hangman.create_game_word_vector(&game_word);

    println!("Guess a letter!");
    for number in (1..tries).rev() {
        let guess = Game::read_user_input();
        //hangman.check_for_letter();
        println!("You guessed {}", guess);
        print!("Keep guessing! ");
        println!("Tries left {}", number);
    }

}

struct Game{
    //game_difficulty: String,
    game_word_array: Vec<char>,
    used_letters: Vec<char>,
}

impl Game{

    fn create_game_word_vector(&mut self, game_word: &String){
        for letter in game_word.chars(){
            self.game_word_array.push(letter);
        }
    }

    fn check_for_letter(&self, input_character: &String){

        for letter in self.game_word_array{
            

        }


    }

    fn read_user_input() -> String{
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something wrong happened");
        buffer = buffer.trim().to_string();
        buffer
    }

}


