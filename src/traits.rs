use config::{is_valid};

pub trait Validatable {
    fn is_valid(&self) -> bool;
}

impl Validatable for i8 {
    fn is_valid(&self) -> bool {
        is_valid(*self)
    }
}
