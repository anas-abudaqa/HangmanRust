use std::io;


fn main() {
    println!("Welcome to Hangman");

    //let mut guess = String::new();
    //let mut WORD_LENGTH: i32 = chosen_word.chars().count();
    let tries = 10;
    let game_word = String::from("mississippi");

    let mut hangman = Game{
        game_word_vec: Vec::new(),
        
        drawn_word_vec: Vec::new(),

        used_letters: Vec::new()
    };

    hangman.create_game_word_vector(&game_word);

    println!("Guess a letter!");
    for number in (1..tries).rev() {


        let guess = Game::read_user_input();

        let indexes = hangman.find_matching_letters(&guess);

        println!("You guessed {}", guess);

        hangman.draw(&indexes, guess);
        
        println!("Tries left {}", number);
    }

}

struct Game{
    //game_difficulty: String,
    //game_word_vec is a char vector that we will compare user input to
    game_word_vec: Vec<char>,

    //drawn_word_vec is the char vector that we will use for visualization and detecting game completion
    drawn_word_vec: Vec<char>,

    //used_letters is a char vector to store all chars inputted by the user
    used_letters: Vec<char>
    

}

impl Game{

    fn read_user_input() -> char{
        //create a buffer string variable to store result of read_line in
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something wrong happened");
        //collect the output in a vec of chars and return first char in it
        let output: Vec<char> = buffer.chars().collect();
        output[0]
    }

    fn create_game_word_vector(&mut self, chosen_word: &String){
        //iterate over letters in chosen word and append them into our vec of chars
        for letter in chosen_word.chars(){
            self.game_word_vec.push(letter);
        }

        self.drawn_word_vec = vec!['_'; self.game_word_vec.len()]
    }

    fn find_matching_letters(&self, input_character: &char) -> Vec<usize>{
        let mut index_vec: Vec<usize> = Vec::new();
        //iterate over game_word_vec using .iter().enumerate() to get letter and its index
        for (index, letter) in self.game_word_vec.iter().enumerate(){
            if input_character == letter{
                //if we find a match, add that index to our index vector
                index_vec.push(index);
            }    
        }

        if index_vec.is_empty(){
            println!("Sorry! Wrong guess. Keep trying");
        }
        else{
            println!("Nice guess!");
        }

        index_vec
    }

    //Note: This function will take ownership of input_character. I was too lazy to implement it elsewise
    fn draw(&mut self, indexes: &Vec<usize>, input_character: char){
        //first check if the array is not empty
        if !indexes.is_empty() {
            for &index in indexes.iter(){
                self.drawn_word_vec[index] = input_character;
            }

        }
        println!("{:?}", self.drawn_word_vec); 

    }



}


