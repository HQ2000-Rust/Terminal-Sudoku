use crate::utils::field_utils::PlayingField;

//can be used if you want to load a standard config or add your own one
pub fn get_template(number: u32) -> Vec<crate::utils::field_utils::PlayingField> {
    #[allow(unused_imports)]
    use crate::{Field, Number};
    use crate::{Field as F, Number as N};
    //space for templates
    match number {
        _ => vec![PlayingField::from([
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
        ])],
    }
}
