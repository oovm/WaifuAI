use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NaiError {
    IOError(std::io::Error)
}

impl Display for NaiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for NaiError {

}

pub type NaiResult<T=()> = Result<T, NaiError>;