use giftwrap::Wrap;

pub type STResult<T> = Result<T, STError>;

#[derive(Debug, Wrap)]
pub enum STError {
    Io(std::io::Error),
    TomlDe(toml::de::Error),
    VarError(std::env::VarError),
    Sway(swayipc::Error),
    Utf8(std::string::FromUtf8Error),
    Other(String),
}
