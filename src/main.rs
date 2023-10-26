use std::io;


fn main() {
    println!("Welcome to Hangman");

    //let mut guess = String::new();
    //let mut WORD_LENGTH: i32 = chosen_word.chars().count();
    let tries = 6;
    let game_word = String::from("Penis");

    let mut hangman = Game{
        game_word_vec: Vec::new(),
        used_letters: Vec::new()
    };
    hangman.create_game_word_vector(&game_word);

    println!("Guess a letter!");
    for number in (1..tries).rev() {
        let guess = Game::read_user_input();
        hangman.check_for_letter(&guess);
        println!("You guessed {}", guess);
        print!("Keep guessing! ");
        println!("Tries left {}", number);
    }

}

struct Game{
    //game_difficulty: String,
    game_word_vec: Vec<char>,
    used_letters: Vec<char>,
}

impl Game{

    fn read_user_input() -> char{
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something wrong happened");
        let mut output: Vec<char> = buffer.chars().collect();
        output[0]
    }


    fn create_game_word_vector(&mut self, game_word: &String){
        for letter in game_word.chars(){
            self.game_word_vec.push(letter);
        }
    }


    fn check_for_letter(&self, input_character: &char){
        for index in &self.game_word_vec.len().iter(){
            println!("{}", index);
        }
        
    }

    

}


