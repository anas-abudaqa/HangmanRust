/* 
Game starts

//////////////
Setting up game:

Read user input for difficulty settings
Choose game word from external file

//////////////
/// 


loop starts:

turn starts

Start with reading user input for guess
Check if input char has already been used
if yes, restart turn

if not, check for matches in game word
if matches, update drawing
if not, tries left -1

turn ends

if drawn vector has all chars, game is win ==================================>Win state
if # of tries done, and drawn vector is not yet full, game is lose ==========>Game over state

if neither, restart loop


*/

use std::io;


fn main() {
    println!("Welcome to Hangman");

    //let mut guess = String::new();
    //let mut WORD_LENGTH: i32 = chosen_word.chars().count();

    let mut hangman = Game{
        turns_left: 0,

        game_word_vec: Vec::new(),
        
        drawn_word_vec: Vec::new(),

        used_letters_vec: Vec::new(),

        game_state: Gamestate::Setup
    };
    

    
    let mut guess = '-'; 
    let mut matching_indices: Vec<usize> = Vec::new();

    loop {


        match hangman.game_state {
            Gamestate::Setup => hangman.setup_game(),
            Gamestate::AwaitingUser => {
                guess = hangman.read_user_input();
                
            },
            //not working anymore idek
            Gamestate::FindingMatches => {
                matching_indices = hangman.find_matching_indices(&guess);
                
            },
            Gamestate::UpdateScreen =>  {
                hangman.draw(&matching_indices, guess);
                hangman.check_endgame_conditions();
            }
            Gamestate::GameEnd(true) => hangman.game_is_won(),
            Gamestate::GameEnd(false) => hangman.game_is_lost()
        }

        
        //let guess = Game::read_user_input();
        //let indices = hangman.find_matching_indices(&guess);
        //println!("You guessed {}", guess);
        //hangman.draw(&indices, guess);
        //println!("Tries left {}", hangman.turns_left);

    }

}



#[derive(Debug, PartialEq)]
enum Gamestate {
    Setup, 
    AwaitingUser,
    FindingMatches,
    UpdateScreen, //hold max turns left? 
    GameEnd(bool), //true is a win, false is a loss
}

struct Game{
    turns_left: u8,

    //game_word_vec is a char vector that we will compare user input to
    game_word_vec: Vec<char>,

    //drawn_word_vec is the char vector that we will use for visualization and detecting game completion
    drawn_word_vec: Vec<char>,

    //used_letters is a char vector to store all chars inputted by the user
    used_letters_vec: Vec<char>,
    
    game_state: Gamestate

}

impl Game{
    

    fn update_gamestate(&mut self, state_number: u8 ){

        match state_number {
            0 => self.game_state = Gamestate::Setup,
            1 => self.game_state = Gamestate::AwaitingUser, 
            2 => self.game_state = Gamestate::FindingMatches,
            3 => self.game_state = Gamestate::UpdateScreen,
            4 => self.game_state = Gamestate::GameEnd(true),
            5 => self.game_state = Gamestate::GameEnd(false),  
            _ => println!("Invalid number, Error updating gamestate!")
        }


    }

    fn print_gamestate(&self) {
        println!(
            "
            Turns left = {}
            Current state is {:?}"
            , self.turns_left, self.game_state
        );
    }


    //////// SETUP Functions
    fn create_game_word_vector(&mut self, chosen_word: &String){
        //iterate over letters in chosen word and append them into our vec of chars
        for letter in chosen_word.chars(){
            self.game_word_vec.push(letter);
        }

        self.drawn_word_vec = vec!['_'; self.game_word_vec.len()]
    }
    

    
    fn setup_game(&mut self){
        let tries = 10;
        let game_word = String::from("mississippi");
        self.turns_left = tries;
        self.create_game_word_vector(&game_word);
        self.update_gamestate(1); //Go to state AwaitingUser
        self.print_gamestate();

        
    }

    

    ///// AWAITING USER Functions
    fn verify_input(&self, input_character: &char) -> bool {
        //if input_character
        let mut available:bool = true;
        for letter in self.used_letters_vec.iter(){
            if input_character == letter{
                available = false;
            }
        }
        available
    }

    fn read_user_input(&mut self) -> char{
        println!("Guess a letter!");
        //create a buffer string variable to store result of read_line in
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something wrong happened");
        //collect the output in a vec of chars and return first char in it
        let output: Vec<char> = buffer.chars().collect();
        let result = self.verify_input(&output[0]);
        
        if !result {
            println!("Sorry, you already used {}", output[0]);
            self.update_gamestate(3); //Go to state: UpdateScreen
            //'*' ////////why do i need this return??
        } else {
            self.used_letters_vec.push(output[0]);
            self.turns_left -= 1; 
            self.update_gamestate(2); //Go to state: FindingMatches
            output[0]
        }

    }



    ///// FindingMatches Functions
    fn find_matching_indices(&mut self, input_character: &char) -> Vec<usize>{
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
        self.update_gamestate(3); //Go to state: UpdateScreen
        // return vector of the indices of the game word where a match was found
        index_vec
    }


    


    ///// UpdateScreen functions
    fn draw(&mut self, indices: &Vec<usize>, input_character: char){
        //first check if the array is not empty
        if !indices.is_empty() {
            for &index in indices.iter(){
                self.drawn_word_vec[index] = input_character;
            }

        }
        println!("{:?}", self.drawn_word_vec); 

    }
    

    fn check_endgame_conditions(&mut self){
        let mut game_won = false;
        let mut game_lost = false;

        for &letter in self.drawn_word_vec.iter(){
            if letter == '_' {
                //if there are _ in drawn_word_vec, then we still have letters to guess
                game_won = false;
                break; //no point in searching more, so break out of loop
            
            } 
            else {game_won = true};//if there is a letter there, set game_won true
        }

        if self.turns_left == 0 {game_lost = true}; //if we run out of turns, set game_lost true

        if game_won {self.update_gamestate(4)} //if game_won is true, go to state: EndGame(Win)
        else if game_lost {self.update_gamestate(5)}//if game_won is false and if game_lost is true, go to state: EndGame(Lost)
        else { self.update_gamestate(1); } //if neither conditions satisfied, go back to state: AwaitingUser

    }

    fn game_is_won(&mut self){
        let mut buffer = String::new();
        println!("CONGRATULATIONS, YOU HAVE WON");
        println!("Play again? (y, n)");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something wrong happened");

        if buffer == "y" {self.update_gamestate(0)}
        else {};

    }


    fn game_is_lost(&mut self){
        let mut buffer = String::new();
        println!("You Lost. You ran out of tries :(");
        println!("Play again? (y, n)");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something wrong happened");

        if buffer == "y" {self.update_gamestate(0)}
        else {};

    }

}


