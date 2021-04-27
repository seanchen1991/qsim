use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// #[derive(Clone, Debug, PartialEq)]
// pub enum Error {}

// impl std::error::Error for Error {}
