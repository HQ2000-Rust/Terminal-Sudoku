use crate::utils::field_utils::{Field, Number, PlayingField, decode};
use crate::utils::general::get_input;
use crate::utils::general::menu::settings::Flags;

mod playing_field_templates;
mod utils;
#[allow(unused_labels)]

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to Terminal Sudoku!");
    let settings = crate::utils::general::menu::settings::get_raw_flags();
    let settings = Flags {
        stopwatch: settings.contains(&("stopwatch".to_string())),
        sudoku_maker: settings.contains(&("sudoku_maker".to_string())),
        templates: settings.contains(&("templates".to_string())),
    };
    game_loop(settings);
}
#[inline]
fn game_loop(settings: Flags) {
    use utils::field_utils::{Field, PlayingField};
    let mut stats = crate::utils::general::stats::Stats::new();
    'round: loop {
        utils::general::menu::general_menu(stats.clone());
        /* replace PlayingField::new() with crate::playing_field_templates::get_template([number assigned to the wanted pattern])
        to use your own standard templates
        use the line under this one to get the empty field
        let mut playing_field = PlayingField::new(); */
        let mut playing_field = playing_field_templates::get_template(rand::random_range(1..=5));
        // saved regardless of the flags to prevent compiler warnings about possibly uninitialized values
        let start = std::time::Instant::now();
        // loop labels: here to make orientation easier, they aren't needed (mostly)
        'turn: loop {
            let mut y_coord_switch = true;
            let mut x_coord: usize = 0;
            let mut y_coord: usize = 0;
            'coords: loop {
                print!("\x1B[2J\x1B[1;1H");
                playing_field.print();
                println!();
                println!("(y: {}, x: ?)",{
                    if y_coord != 0 {
                        y_coord.to_string()
                    } else {
                        "?".to_string()
                    }
                });
                println!("Which field do you want to change?");
                println!("Input the {}-coordinate and press ENTER:", {
                    if y_coord_switch { "y" } else { "x" }
                });
                match get_input().trim().parse::<usize>() {
                    Err(_) => {
                        println!("Please enter a number between 1 and 9 (inclusive)!");
                        continue 'coords;
                    }
                    Ok(number) => {
                        if !(number >= 1 && number <= 9) {
                            println!("Please enter a number between 1 and 9 (inclusive)!");
                            continue 'coords;
                        }
                        match y_coord_switch {
                            true => {
                                if playing_field.y_has_empty(number) {
                                    y_coord = number;
                                }
                                else {
                                    println!("That y coordinate has no empty spaces! (Press ENTER)");
                                    get_input();
                                    y_coord_switch = true;
                                    continue 'coords;
                                }
                            }
                            false => {
                                x_coord = number;
                        
                            }
                        }
                        if !y_coord_switch {
                            let field = playing_field.access(x_coord, y_coord);
                            if !field.eq(&Field::Empty) {
                                println!(
                                    "You need an empty Field to put something in! (Press ENTER)"
                                );
                                get_input();
                                y_coord_switch = true;
                                continue 'coords;
                            }
                            break 'coords;
                        }
                        y_coord_switch = false;
                    }
                }
            }
            let field_type_i32 = loop {
                print!("\x1B[2J\x1B[1;1H");
                playing_field.print();
                println!();
                println!("(y: {}, x: {})",y_coord, x_coord);
                println!(
                    "Which number should be inserted? Type it in and press ENTER."
                );
                if let Ok(number) = get_input().trim().parse::<i32>() {
                    if number >= 1 && number <= 9 {
                        break number;
                    }
                }
                println!("Please enter a number between 1 and 9 (inclusive)!");
            };
            playing_field.set(
                x_coord,
                y_coord,
                Field::Number({
                    crate::utils::field_utils::i32_to_number(&field_type_i32)
                        .expect("already bound and type checked, so this should be impossible")
                }),
            );
            print!("\x1B[2J\x1B[1;1H");
            playing_field.print();
            println!("Press ENTER to continue.");
            get_input();
            print!("\x1B[2J\x1B[1;1H");
            if settings.templates {
                'template: loop {
                    println!("Do you want to apply a template? (y/n)");
                    match get_input().trim() {
                        "y" => {
                            println!("Please input a saved state code and press ENTER to apply it");
                            match decode(&get_input().trim().to_string()) {
                                None => {
                                    println!("Invalid state code!");
                                    println!("Do you want to apply a state code? (y/n)");
                                    match get_input().trim() {
                                        "y" => {
                                            println!("Ok, so");
                                            continue 'template;
                                        }
                                        _ => break 'template,
                                    }
                                }
                                Some(field) => {
                                    playing_field = field;
                                    println!("Applied template!");
                                }
                            }
                        }
                        _ => break 'template,
                    }
                }
            }
            print!("\x1B[2J\x1B[1;1H");
            if settings.sudoku_maker {
                'save: loop {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("Do you want to save this state? (y/n)");
                    match get_input().trim().parse::<i32>() {
                        Ok(1) => {
                            println!("Nice! Here is your code to copy:");
                            println!("{}", playing_field.encode());
                            println!("(Press ENTER to continue with the game)");
                        }
                        _ => {
                            println!("Fine. Good luck and have fun! (Press ENTER)");
                            get_input();
                            break 'save;
                        }
                    }
                }
            }
            use crate::utils::field_utils::SolvingState;
            match playing_field.solving_state() {
                SolvingState::Solvable => {}
                SolvingState::Unsolvable => {
                    stats.add_lost();
                    print!("\x1B[2J\x1B[1;1H");
                    playing_field.print();
                    println!("It's unsolvable now! (Press ENTER)");
                    get_input();
                    break 'turn;
                }
                SolvingState::Solved => {
                    stats.add_won();
                    println!("Solved!");
                    if settings.stopwatch {
                        println!(
                            "You needed {}m and {}s!",
                            (start.elapsed().as_secs() - (start.elapsed().as_secs() % 60)) / 60,
                            start.elapsed().as_secs() % 60
                        );
                    }

                    if stats.fastest_run == None {
                        stats.fastest_run = Some(start.elapsed());
                        println!("That's your first and fastest run! (With stopwatch enabled)")
                    } else {
                        if start.elapsed()
                            > stats
                                .fastest_run
                                .expect("Impossible to reach (at least logically) since the None case got covered")
                        {
                            stats.fastest_run = Some(start.elapsed());
                            println!("That's a new best!");
                        }
                    }
                    println!("Press ENTER");
                    get_input();
                    break 'turn;
                }
            }
        }
    }
}
