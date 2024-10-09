#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(String),
}
