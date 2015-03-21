// range constants.
pub const MIN_POS: i8 = 1;
pub const MAX_POS: i8 = 10;

pub fn full_range() -> Vec<i8> {
    let mut range = vec![];
    for i in MIN_POS..MAX_POS {
        range.push(i);
    }
    range
}

pub fn is_valid(value: i8) -> bool {
    (value >= MIN_POS) && (value < MAX_POS)
}

pub fn assert_valid_value(value: i8) -> i8 {
    assert!(is_valid(value));
    value
}
