use crate::utils::field_utils::{Field, Number, PlayingField};
use std::io::{Read, Write};

mod utils;

fn main() {
    println!("Welcome to Terminal Sudoku!");

    /*let field=PlayingField::from([[Field::Empty;9],
        [Field::Empty;9],
        [Field::Number(Number::Seven);9],
        [Field::Empty;9],
        [Field::Number(Number::Nine);9],
        [Field::Empty;9],
        [Field::Empty;9],
        [Field::Empty;9],
        [Field::Empty;9],]);
    field.print();*/

    game_loop();
}

#[inline]
fn game_loop() {
    use utils::field_utils::{Field, PlayingField};
    loop {
        utils::general::menu::general_menu();
        let mut playing_field = PlayingField::new();
    }
}
