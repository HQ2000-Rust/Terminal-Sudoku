use crate::utils::field_utils::PlayingField;

fn get_templates() -> Vec<crate::utils::field_utils::PlayingField> {
    use crate::{Field, Number};
    vec![PlayingField::from([
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
        [Field::Empty; 9],
    ])]
}
