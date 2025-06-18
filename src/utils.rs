pub mod field_utils {
    use std::fmt::Display;

    #[derive(PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
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

    #[derive(PartialEq, PartialOrd, Copy, Clone, Ord, Eq)]
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

    pub fn i32_to_Number(value: i32) -> Option<Number> {
        match value {
            0 => Some(Number::One),
            1 => Some(Number::Two),
            2 => Some(Number::Three),
            3 => Some(Number::Four),
            4 => Some(Number::Five),
            5 => Some(Number::Six),
            6 => Some(Number::Seven),
            7 => Some(Number::Eight),
            8 => Some(Number::Nine),
            9 => Some(Number::Nine),
            _ => None,
        }
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

        //returns false when this operation is not possible, x-/y-coords are *NOT* 0-indexed (likely unused)
        fn set_checked(&mut self, x_coord: usize, y_coord: usize, field: &Field) -> FieldSetResult {
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
                println!("{sep1}");
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
            println!("2. Settings");
            println!("3. Quit");
            println!("Type the corresponding number and press ENTER to choose an option");
            match get_input().trim().parse::<i32>() {
                Ok(1) => {
                    //nothing, so it continues with the game
                }
                Ok(2) => {
                    settings::settings_menu();
                }
                //Typing in "3" or anything else means quit
                _ => {
                    quit_menu();
                }
            }
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
        pub mod settings {
            use crate::utils::general::get_input;

            #[derive(Debug, Default)]
            pub struct Flags {
                pub stopwatch: bool,
                pub sudoku_maker: bool,
                pub templates: bool,
            }

            pub fn get_raw_flags() -> Vec<String> {
                let mut active_flags = Vec::new();
                for flag in ["stopwatch", "sudoku_maker", "templates"] {
                    if std::env::args().collect::<String>().contains(flag) {
                        active_flags.push(flag.to_string());
                    }
                }
                active_flags
            }

            #[inline]
            pub fn settings_menu() {
                println!("-Settings-");
                println!(
                    "To apply settings, you can run this exe with some of the following flags:"
                );
                println!(
                    " --stopwatch -> enables a stopwatch that tracks how long you need to solve the Sudoku"
                );
                println!(
                    " --sudoku-maker -> asks you to save your field after every turn as a string you can later use as a template"
                );
                println!(
                    " --template -> asks you to use a template you obtained with --sudoku-maker before every round, you can also select a standard template (e.g. an empty field, which you normally can't obtain)"
                );
                println!("The active flags are:");
                for flag in get_raw_flags() {
                    println!(" --{}", flag);
                }
                println!("Press ENTER to continue");
                get_input();
            }
        }
    }
}
