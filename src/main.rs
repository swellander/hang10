use std::io;

fn main() {
    let word = "grapefruit";
    let mut game = Game::new(word).unwrap();

    println!("Welcome to Hang 10!");
    println!(
        "Your word is {} letters long. You have 10 guesses. Go!",
        game.target.len()
    );

    while game.is_on {
        game.play_turn();
    }

    if game.is_lost() {
        println!("Bit the farm...");
    } else {
        println!("Radical dude!");
    }
}

#[derive(Debug)]
struct Game {
    target: String,
    guess_state: Vec<char>,
    remaining_guesses: u8,
    is_on: bool,
}

impl Game {
    fn new(word: &str) -> Result<Game, std::io::Error> {
        Ok(Game {
            target: word.to_lowercase().to_string(),
            guess_state: vec!['_'; word.len()],
            remaining_guesses: 10,
            is_on: true,
        })
    }

    fn play_turn(&mut self) {
        if self.is_lost() {
            self.is_on = false;
        } else {
            self.await_guess();
        }
    }

    fn await_guess(&mut self) {
        let guess_str: String = self
            .guess_state
            .iter()
            .map(|x| format!("{}", x.to_string() + " "))
            .collect();

        println!("{} guesses remaining", self.remaining_guesses);
        println!("{}", guess_str.trim());

        let mut guess = String::new(); // TODO: limit to single char
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess input.");

        self.handle_guess(guess.to_lowercase().trim());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn handle_guess(&mut self, guess: &str) {
        let is_hit = self.target.contains(guess);
        if is_hit {
            self.handle_hit(guess);
        } else {
            self.handle_miss();
        }
    }

    fn handle_hit(&mut self, guess: &str) {
        let mut indicies: Vec<usize> = vec![];
        for (i, letter) in self.target.chars().enumerate() {
            if letter == guess.chars().next().unwrap() {
                indicies.push(i)
            }
        }
        for i in indicies {
            self.guess_state[i] = guess.chars().next().unwrap();
        }

        if self.is_won() {
            self.is_on = false;
        }
    }

    fn handle_miss(&mut self) {
        self.remaining_guesses -= 1;
    }

    fn is_lost(&self) -> bool {
        self.remaining_guesses == 0
    }

    fn is_won(&self) -> bool {
        !self.guess_state.contains(&'_')
    }
}
