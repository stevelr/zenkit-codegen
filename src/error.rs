use std::fmt;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Zenkit(String),
    //NoApi,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{:?}",
            match self {
                Error::Message(s) => s,
                Error::Zenkit(s) => s,
            }
        )
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(e: handlebars::TemplateError) -> Error {
        println!("Template Error: {:#?}", e);
        Error::Message(e.to_string())
    }
}
impl From<handlebars::RenderError> for Error {
    fn from(e: handlebars::RenderError) -> Error {
        println!("Render Error: {:#?}", e);
        Error::Message(e.to_string())
    }
}
impl From<zenkit::Error> for Error {
    fn from(e: zenkit::Error) -> Error {
        Error::Zenkit(e.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Message(e.to_string())
    }
}

impl std::error::Error for Error {}
