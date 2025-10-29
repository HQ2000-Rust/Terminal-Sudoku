pub mod field_utils {
    use std::collections::HashSet;
    use std::fmt::Display;

    pub fn decode(string: &String) -> Option<PlayingField> {
        let mut result = [[Field::Empty; 9]; 9];
        for row in 0..=8 {
            for field in 0..=8 {
                result[row][field] = match string.chars().nth(row * 9 + field).unwrap() {
                    'E' => Field::Empty,
                    '1' => Field::Number(Number::One),
                    '2' => Field::Number(Number::Two),
                    '3' => Field::Number(Number::Three),
                    '4' => Field::Number(Number::Four),
                    '5' => Field::Number(Number::Five),
                    '6' => Field::Number(Number::Six),
                    '7' => Field::Number(Number::Seven),
                    '8' => Field::Number(Number::Eight),
                    '9' => Field::Number(Number::Nine),
                    _ => {
                        return None;
                    }
                }
            }
        }
        Some(PlayingField::from(result))
    }
    #[derive(PartialEq, Copy, Clone, Ord, PartialOrd, Eq, Hash, Debug)]
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

    #[derive(PartialEq, PartialOrd, Copy, Clone, Ord, Eq, Debug, Hash)]
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

    //done here to show that it's a conversion from an i32 to a Number enum variant, "number_to_number" would be unclear
    pub fn i32_to_number(value: &i32) -> Option<Number> {
        match value {
            1 => Some(Number::One),
            2 => Some(Number::Two),
            3 => Some(Number::Three),
            4 => Some(Number::Four),
            5 => Some(Number::Five),
            6 => Some(Number::Six),
            7 => Some(Number::Seven),
            8 => Some(Number::Eight),
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

    #[derive(Debug)]
    pub enum SolvingState {
        Solvable,
        Unsolvable,
        Solved,
    }

    #[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
    pub struct PlayingField {
        field: [[Field; 9]; 9],
    }

    impl PlayingField {
        //used for templates
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

        // functions to check if the y or x coordinate has an empty space
        pub fn y_contains_empty(&self, y_coord: usize) -> bool {
            let mut x_coord: usize = 1;
            loop {
                if x_coord < 10 {
                    let field = self.access(x_coord, y_coord);
                    if field.eq(&Field::Empty) {
                        return true;
                    }
                    x_coord += 1;
                } else {
                    return false;
                }
            }
        }

        pub fn print(&self) {
            let sep1 = "\n  -- -- -- -- -- -- -- -- -- --";
            let sep2 = " |  ";
            print!("x   1 2 3     4 5 6     7 8 9");
            print!("{sep1}");
            //chunk = 3 horizontal rows
            for chunk in 1..=3 {
                //row = horizontal row
                for row in 1..=3 {
                    println!();
                    //frac1=third part of a horiz. row
                    for frac1 in 1..=3 {
                        print!("{sep2}");
                        //field = field
                        for field in 1..=3 {
                            print!(
                                "{} ",
                                self.access((frac1 - 1) * 3 + field, (chunk - 1) * 3 + row)
                            )
                        }
                    }
                    print!("{sep2}");
                    print!("{}", (chunk - 1) * 3 + row);
                }
                print!("{sep1}");
            }
            println!("   y");
        }

        pub fn solving_state(&self) -> SolvingState {
            use super::field_utils::Field::Number as F;
            use super::field_utils::Number as N;
            let pattern: HashSet<Field> = HashSet::from([
                F(N::One),
                F(N::Two),
                F(N::Three),
                F(N::Four),
                F(N::Five),
                F(N::Six),
                F(N::Seven),
                F(N::Eight),
                F(N::Nine),
            ]);
            //horizontal correctness
            for row_counter in 0..=8 {
                let mut y_pattern = pattern.clone();
                let row = self.field[row_counter].to_vec();
                for field in row.iter() {
                    if *field != Field::Empty {
                        match y_pattern.remove(field) {
                            true => continue,
                            false => return SolvingState::Unsolvable,
                        }
                    }
                }
            }

            //vertical correctness
            for vert_row in 0..=8 {
                let mut row = Vec::new();
                for i in 0..=8 {
                    row.push(self.field[i][vert_row]);
                }
                let mut x_pattern = pattern.clone();
                for field in row.iter() {
                    if *field != Field::Empty {
                        match x_pattern.remove(field) {
                            true => continue,
                            false => return SolvingState::Unsolvable,
                        }
                    }
                }
            }

            //square correctness
            for x_squares in 0..=2 {
                for y_squares in 0..=2 {
                    let mut pattern = pattern.clone();
                    for x in 0..=2 {
                        for y in 0..=2 {
                            let field = &self.field[x_squares * 3 + x][y_squares * 3 + y];
                            if *field != Field::Empty {
                                match pattern.remove(field) {
                                    true => continue,
                                    false => return SolvingState::Unsolvable,
                                }
                            }
                        }
                    }
                }
            }
            //checking if it's full -> solved because it got checked for illegal patterns
            let mut is_solved = true;
            for row in self.field {
                for field in row.iter() {
                    if *field == Field::Empty {
                        is_solved = false;
                    }
                }
            }
            if is_solved {
                return SolvingState::Solved;
            }
            SolvingState::Solvable
        }

        //using char instead of string would require serious reformatting or endless conversion chains...
        pub fn encode(&self) -> String {
            let mut result: Vec<String> = Vec::new();
            for row in 0..=8 {
                for field in 0..=8 {
                    match self.field[row][field] {
                        Field::Empty => {
                            result.push("E".to_string());
                        }
                        Field::Number(n) => {
                            result.push(n.to_string());
                        }
                    }
                }
            }
            result.join("")
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

    pub mod stats {

        #[derive(Copy, Clone)]
        pub struct Stats {
            pub won: u32,
            pub lost: u32,
            pub fastest_run: Option<std::time::Duration>,
        }
        impl Stats {
            pub fn new() -> Stats {
                Stats {
                    won: 0,
                    lost: 0,
                    fastest_run: None,
                }
            }
            pub fn add_lost(&mut self) {
                self.lost += 1;
            }
            pub fn add_won(&mut self) {
                self.won += 1;
            }
        }
    }

    pub mod menu {
        use crate::utils::general::get_input;
        use crate::utils::general::stats::Stats;

        //use crate::utils;
        pub fn general_menu(stats: Stats) {
            print!("\x1B[2J\x1B[1;1H");
            println!("-Menu-");
            println!("1. Play Sudoku");
            println!("2. Settings / Information");
            println!("3. Quit");
            println!("Type the corresponding number and press ENTER to choose an option");
            match get_input().trim().parse::<i32>() {
                Ok(1) => {
                    //nothing, so it continues with the game
                }
                Ok(2) => {
                    settings::settings_menu(stats);
                }
                //Typing in "3" or anything else means quit
                _ => {
                    quit_menu(stats);
                }
            }
        }

        #[inline]
        fn quit_menu(stats: Stats) {
            print!("\x1B[2J\x1B[1;1H");
            println!("Do you want to quit? (y/n)");
            if get_input().trim().to_lowercase() == "y" {
                print!("\x1B[2J\x1B[1;1H");
                println!(
                    "You played {} games ({} lost / {} won)",
                    stats.won + stats.lost,
                    stats.lost,
                    stats.won
                );
                match stats.fastest_run {
                    None => {}
                    Some(time) => println!(
                        "Your best run was {}m {}s",
                        time.as_secs() - (time.as_secs() % 60),
                        time.as_secs() % 60
                    ),
                }
                println!();
                println!("'til the next time! (Press ENTER)");
                get_input();
                print!("\x1B[2J\x1B[1;1H");
                eprintln!("Process got terminated by quitting");
                std::process::exit(0);
            }
            general_menu(stats);
        }
        pub mod settings {
            use crate::utils::general::get_input;
            use crate::utils::general::menu::general_menu;
            use crate::utils::general::stats::Stats;

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
            pub fn settings_menu(stats: Stats) {
                print!("\x1B[2J\x1B[1;1H");
                println!("-Settings / Information-");
                println!(
                    "To apply settings, you can run this exe with some of the following flags:"
                );
                println!(
                    " --stopwatch -> enables a stopwatch that tracks how long you need to solve the Sudoku"
                );
                println!(
                    " --sudoku_maker -> asks you to save your field after every turn as a string you can later use as a template"
                );
                println!(
                    " --template -> asks you to use a template you obtained with --sudoku_maker before every round, you can also select a standard template (e.g. an empty field, which you normally can't obtain)"
                );
                println!("All other flags are ignored");
                println!("The active flags are:");
                for flag in get_raw_flags() {
                    println!(" {}", flag);
                }
                if get_raw_flags().is_empty() {
                    println!(" None");
                }
                println!("Your current stats are:");
                println!("{} games played", stats.lost + stats.won);
                println!("{} loses", stats.lost);
                println!("{} wins", stats.won);
                match stats.fastest_run {
                    None => {
                        println!("Currently no fastest time.")
                    }
                    Some(time) => {
                        println!(
                            "Fastest time: {}m {}s",
                            (time.as_secs() - (time.as_secs() % 60)) / 60,
                            time.as_secs() % 60
                        )
                    }
                }
                println!();
                println!("Press ENTER to continue");
                get_input();
                general_menu(stats)
            }
        }
    }
}
