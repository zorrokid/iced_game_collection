use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
pub struct Game {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct System {
    pub id: i32,
    pub name: String,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
