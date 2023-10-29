// A trait which acts similar to expect() but will only print the message given and quit.

pub trait ExpectPretty<T> {
    fn expect_p(self, msg: &str) -> T;
}

impl<T, E> ExpectPretty<T> for Result<T, E> {
    fn expect_p(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                eprintln!("{}", msg);
                std::process::exit(-1)
            }
        }
    }
}

impl<T> ExpectPretty<T> for Option<T> {
    fn expect_p(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                eprintln!("{}", msg);
                std::process::exit(-1)
            }
        }
    }
}