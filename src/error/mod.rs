

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    IO(std::io::Error),
    Yaml(serde_yaml::Error),
    Lua(String),
}
pub type Result<T> = std::result::Result<T, Error>;
impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}
impl std::convert::From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::Yaml(e)
    }
}
