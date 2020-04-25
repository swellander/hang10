fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal screen

    let word = "grapefruit";
    let mut game = Game::new(word).unwrap();

    while game.is_on {
        game.play_turn();
    }

    if game.is_lost() {
        println!("Bit the farm...");
    } else {
        println!("Radical dude!");
    }
}

struct Game {
    target: String,
    guess_state: Vec<char>,
    stages: Vec<String>,
    remaining_guesses: u8,
    is_on: bool,
}

impl Game {
    fn new(word: &str) -> Result<Game, std::io::Error> {
        Ok(Game {
            target: word.to_lowercase().to_string(),
            guess_state: vec!['_'; word.len()],
            remaining_guesses: 10,
            stages: Game::get_surfer_stages()?,
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

        let title = std::fs::read_to_string("src/title.ascii").expect("Couldn't read game title");
        let stage_idx = usize::from(self.remaining_guesses - 1);
        let stage = &self.stages[stage_idx];

        println!("{}", title);
        println!(
            "Your word is {} letters long. You have {} guesses remaining 🤙 \n\n",
            self.target.len(),
            self.remaining_guesses
        );
        println!("{}", stage);
        println!("{}\n\n", guess_str.trim());

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess input.");

        self.handle_guess(guess.to_lowercase().trim());

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal screen
    }

    fn handle_guess(&mut self, guess: &str) {
        if guess.len() == 1 {
            let is_hit = self.target.contains(guess);
            if is_hit {
                self.handle_hit(guess);
            } else {
                self.handle_miss();
            }
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

    pub fn get_surfer_stages() -> std::io::Result<Vec<String>> {
        let mut file_paths = std::fs::read_dir("src/surfer_art")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        file_paths.sort();

        let mut stages = vec![];
        for path in file_paths {
            let content = std::fs::read_to_string(path)?;
            stages.push(content);
        }

        Ok(stages)
    }
}
