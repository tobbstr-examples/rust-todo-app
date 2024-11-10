use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display; // Add this line to import the Display trait
use std::fmt::Formatter;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub enum VarValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    String(String),
}

#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub value: VarValue,
}

#[derive(Debug)]
pub enum Error {
    Root(Root),
    Wrapped(Wrapped),
}

impl Error {
    /// new creates a new default root error with the given message.
    pub fn new(message: String) -> Self {
        Error::Root(Root {
            message,
            log_level: LogLevel::Error,
            runtime_state: Vec::new(),
            source: None,
        })
    }

    pub fn from_source(src: Box<dyn StdError>, message: String) -> Self {
        Error::Root(Root {
            message,
            log_level: LogLevel::Error,
            runtime_state: Vec::new(),
            source: Some(src),
        })
    }

    /// wrap wraps the current error in a new error with the given message.
    pub fn wrap(self, message: String) -> Self {
        Error::Wrapped(Wrapped {
            message,
            error: Box::new(self),
        })
    }

    pub fn add_var(mut self, name: String, value: VarValue) -> Self {
        match self {
            Error::Root(ref mut root) => {
                root.runtime_state.push(Var { name, value });
            }
            _ => match unwrap_mut(&mut self) {
                Some(root) => {
                    root.runtime_state.push(Var { name, value });
                }
                None => {}
            },
        }
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Root(root) => {
                match &root.source {
                    Some(src) => {
                        _ = write!(f, "{}: {}\n", root.message, src);
                    }
                    None => {
                        _ = write!(f, "{}\n", root.message);
                    }
                }
                for var in &root.runtime_state {
                    write!(f, "{}: {:?}\n", var.name, var.value);
                }
                Ok(())
            }
            Error::Wrapped(wrapped) => {
                let _ = write!(f, "{}\n", wrapped.message);
                let mut current_error: &dyn StdError = wrapped.error.as_ref();
                while let Some(src) = current_error.source() {
                    if let Some(err) = src.downcast_ref::<Error>() {
                        match err {
                            Error::Root(root) => {
                                let _ = write!(f, ": {}\n", root.message);
                                if root.runtime_state.len() > 0 {
                                    let _ = write!(f, "\nRuntime State:\n");
                                    for var in &root.runtime_state {
                                        write!(f, "\t{}: {:?}\n", var.name, var.value);
                                    }
                                }
                            }
                            Error::Wrapped(wrapped) => {
                                let _ = write!(f, ": {}\n", wrapped.message);
                            }
                        }
                        current_error = err;
                    } else {
                        break;
                    }
                }
                Ok(())
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Root(root) => match &root.source {
                Some(src) => Some(src.as_ref()),
                None => None,
            },
            Error::Wrapped(wrapped) => Some(&wrapped.error),
        }
    }
}

#[derive(Debug)]
struct Root {
    message: String,
    log_level: LogLevel,
    runtime_state: Vec<Var>,
    source: Option<Box<dyn StdError>>,
}

#[derive(Debug)]
struct Wrapped {
    message: String,
    error: Box<Error>,
}

fn unwrap_mut(err: &mut Error) -> Option<&mut Root> {
    let mut current_error = err;
    while let Error::Wrapped(ref mut e) = current_error {
        current_error = e.error.as_mut();
    }
    match current_error {
        Error::Root(root) => Some(root),
        Error::Wrapped(_) => None,
    }
}
