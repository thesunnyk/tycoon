
use std::fmt::Debug;

pub trait FatalAction<T, E> {
    fn or_die(self, s: &str) -> T;
}

impl<T, E: Debug> FatalAction<T, E> for Result<T, E> {
    fn or_die(self, s: &str) -> T {
        print!("Trying to {}...", s);
        match self {
            Ok(t) => {
                println!("OK");
                t
            },
            Err(e) => {
                panic!("ERROR: {:?}", e);
            }
        }
    }
}

pub trait ResultMap<T, E> {
    fn res_map<U, F>(self, op: F) -> Result<U, E>
        where F: FnOnce(T) -> Result<U, E>;
}

impl<T, E> ResultMap<T, E> for Result<T, E> {
    fn res_map<U, F>(self, op: F) -> Result<U, E>
        where F: FnOnce(T) -> Result<U, E> {
        match self {
            Err(x) => Err(x),
            Ok(x) => op(x)
        }
    }
}

