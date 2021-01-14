use std::fmt;
use std::io;

pub enum GitCliError {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
}

impl fmt::Display for GitCliError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &GitCliError::IoError(ref e) => e.fmt(formatter),
            &GitCliError::NoDirectory => formatter.write_str("No Directory Found"),
            &GitCliError::InvalidCommit => formatter.write_str("Invalid commit."),
            &GitCliError::InvalidIndex => formatter.write_str("Invalid index"),
        }
    }
}

impl From<io::Error> for GitCliError {
    fn from(err: io::Error) -> GitCliError {
        GitCliError::IoError(err)
    }
}
