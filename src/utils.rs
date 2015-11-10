
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

