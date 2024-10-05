use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub releases: Vec<Release>,
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

#[derive(Debug, Clone)]
pub struct Release {
    pub id: i32,
    pub name: String,
    pub system_id: i32,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
