pub mod field_utils {
    use std::fmt::Display;

    #[derive(PartialEq, Copy, Clone)]
    pub enum Field {
        Empty,
        Number(Number),
    }

    impl Display for Field {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", {
                match self {
                    Field::Empty => " ".to_string(),
                    Field::Number(n) => n.to_string(),
                }
            })
        }
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum Number {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
    }
    impl Display for Number {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", {
                match self {
                    Number::One => "1",
                    Number::Two => "2",
                    Number::Three => "3",
                    Number::Four => "4",
                    Number::Five => "5",
                    Number::Six => "6",
                    Number::Seven => "7",
                    Number::Eight => "8",
                    Number::Nine => "9",
                }
            })
        }
    }

    pub enum SolvingState {
        Solvable,
        Unsolvable,
        Solved,
    }

    enum FieldSetResult {
        Success,
        OutOfBounds,
        AlreadySet,
    }

    pub struct PlayingField {
        field: [[Field; 9]; 9],
    }
    impl PlayingField {
        pub fn new() -> Self {
            PlayingField {
                field: [[Field::Empty; 9]; 9],
            }
        }

        //temporary debug function
        pub fn from(field: [[Field; 9]; 9]) -> Self {
            PlayingField { field }
        }

        //does panic on operation out of bounds!
        //1-indexed!
        pub fn access(&self, x_coord: usize, y_coord: usize) -> &Field {
            &self.field[x_coord - 1][y_coord - 1]
        }

        //same rules as for PlayingField::access ^^^^^
        pub fn set(&mut self, x_coord: usize, y_coord: usize, field: Field) {
            self.field[x_coord - 1][y_coord - 1] = field;
        }

        //returns false when this operation is not possible, x-/y-coords are *NOT* 0-indexed
        fn set_field(&mut self, x_coord: usize, y_coord: usize, field: &Field) -> FieldSetResult {
            if x_coord <= 9 && y_coord <= 9 {
                let mut target_field = self.access(x_coord, y_coord);
                if *target_field == Field::Empty {
                    self.set(x_coord, y_coord, Field::Empty);
                    return FieldSetResult::Success;
                }
                return FieldSetResult::AlreadySet;
            }
            FieldSetResult::OutOfBounds
        }

        pub fn print(&self) {
            let sep1 = "\n  -- -- -- -- -- -- -- -- -- --";
            let sep2 = " |  ";
            print!("{sep1}");
            //chunk = 3 horizontal rows
            for chunk in 1..=3 {
                //row = horizontal row
                for row in 1..=3 {
                    println!();
                    //frac1=third part of a horiz. row
                    for frac1 in 1..=3 {
                        print!(" |  ");
                        //field = field
                        for field in 1..=3 {
                            print!(
                                "{} ",
                                self.access((chunk - 1) * 3 + row, (frac1 - 1) * 3 + field)
                            )
                        }
                    }
                    print!("{sep2}");
                }
                print!("{sep1}");
            }

            pub fn solving_state() -> SolvingState {
                {
                    use super::field_utils::Field::Number as F;
                    use super::field_utils::Number as N;
                    let pattern = [
                        F(N::One),
                        F(N::Two),
                        F(N::Three),
                        F(N::Four),
                        F(N::Five),
                        F(N::Six),
                        F(N::Seven),
                        F(N::Eight),
                        F(N::Nine),
                    ];
                }
                for i in 0..9 {}
                SolvingState::Solvable
            }
        }
    }
}

pub mod general {
    pub fn get_input() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("I/O needs to work in order to play this game!");
        input
    }

    pub mod menu {
        use crate::utils::general::get_input;

        //use crate::utils;
        pub fn general_menu() {
            println!("-Menu-");
            println!("1. Play Sudoku");
            println!("2. View the leaderboard");
            println!("3. Settings");
            println!("4. Quit");
            println!("Type the corresponding number and press ENTER to choose an option");
            match get_input().trim().parse::<i32>() {
                Ok(1) => {
                    //nothing, so it continues with the game
                }
                Ok(2) => {
                    leaderboard_menu();
                }
                Ok(3) => {
                    settings::settings_menu();
                }
                //Typing in "4" or anything else mean quit
                _ => {
                    quit_menu();
                }
            }
        }
        #[inline]
        fn leaderboard_menu() {
            println!("-Leaderboard-");
            todo!("leaderboard impl");
        }

        #[inline]
        fn breakup_menu() {
            println!("-Breakup if the game is unsolvable-");
        }

        #[inline]
        fn quit_menu() {
            println!("Do you want to quit? (y/n)");
            if get_input().trim().to_lowercase() == "y" {
                println!("'til the next time! (Press ENTER)");
                get_input();
                eprintln!("Process got terminated by quitting");
                std::process::exit(0);
            }
            general_menu();
        }
        mod settings {
            use std::io::BufRead;

            enum ChangeSetting {
                BreakupOnUnsolvable(bool),
                Stopwatch(bool),
            }
            struct Settings {
                breakup_on_unsolvable: bool,
                stopwatch: bool,
            }
            impl Default for Settings {
                fn default() -> Self {
                    Settings {
                        breakup_on_unsolvable: true,
                        stopwatch: true,
                    }
                }
            }
            static SETTINGS_ENV_VAR: &str = "TERMINAL_SUDOKU_SETTINGS";

            #[inline]
            pub fn settings_menu() {
                println!("-Settings-");
                println!("Type in the corresponding number and press ENTER to change a setting");
                println!("1. Breakup if the game is unsolvable");
                println!("2. Stopwatch");
                println!("3. Save settings in an environment variable");
                println!("4. Exit the settings");
                match super::get_input().trim().parse::<i32>() {
                    Ok(1) => {}
                    Ok(2) => {}
                    Ok(3) => {}
                    _ => {}
                }
            }
            
            fn to_env_settings(settings: Settings) {
                
            }

            fn edit_setting(setting: ChangeSetting) {
                let settings = Settings::default();
                if let Some(s) = get_settings() {
                    
                }
            }
            fn parse_settings(string: String) -> Settings {
                let strings = string.split(",").collect::<Vec<&str>>();
                use std::collections::HashMap;
                let mut settings: HashMap<&str, Option<bool>> = HashMap::new();
                for string in strings {
                    let mut pair = string.split(":").collect::<Vec<&str>>();

                    settings.insert(
                        pair[0],
                        match pair[1] {
                            "true" => Some(true),
                            "false" => Some(false),
                            _ => None,
                        },
                    );
                }
                let mut result = Settings::default();
                let mut changed = false;
                //double some since env filtering and hashmap returns it
                if let Some(Some(b)) = settings.get("breakup_on_unsolvable") {
                    result.breakup_on_unsolvable = *b;
                }
                if let Some(Some(b)) = settings.get("breakup_on_unsolvable") {
                    result.stopwatch = *b;
                }
                result
            }

            fn get_settings() -> Option<String> {
                let mut settings = None;
                for env_pair in std::env::vars() {
                    let (key, value) = env_pair;
                    if key == SETTINGS_ENV_VAR {
                        settings = Some(value);
                    }
                }
                settings
            }
        }
    }
}
