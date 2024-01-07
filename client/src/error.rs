use std::io;

macro_rules! multi_error {
    ($name:ident($($manual:ident),*); $($err:ident = $obj:ty);*) => {
        #[derive(Debug)]
        pub enum $name {
            $($err($obj),)*
            $($manual),*
        }
        
        impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(self, f) } }
        impl std::error::Error for Error {}
    
        $(impl From<$obj> for $name { fn from(value: $obj) -> Self { Self::$err(value) } })*
        
    }
}

/// Each line represents a possible error type, acting as a union between all error types below. 
/// This is especially useful when making heavy use of the ? operator, as any Result type whose error type is listed below can be coerced into the defined type.
multi_error! { Error();
    IoError = io::Error
}

pub type Result<T> = ::std::result::Result<T, Error>;
