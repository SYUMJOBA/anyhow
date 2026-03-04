#[derive(Clone, Debug)]
pub enum Error {
    InvalidIndex,
    InvalidFormat,
    AlreadyPresent,
    NotFound
}
