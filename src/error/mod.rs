use serde::{Deserialize, Serialize, Serializer,Deserializer};
use serde_json;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Yaml(serde_yaml::Error),
    Lua(String),
    SQLite(rusqlite::Error),
    JSON(serde_json::Error),
    Normal(String),
    Utf8Error(std::str::Utf8Error),
    Mustache(mustache::Error),
    OK,
}
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = format!("{}", self);
        return serializer.serialize_str(&data);
    }
}

impl std::fmt::Display for Error{
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
    match self {
        Error::OK=>write!(f,"No error"),
        Error::JSON(e)=>write!(f,"{}",e),
        Error::IO(e)=>write!(f,"{}",e),
        Error::Yaml(e)=>write!(f,"{}",e),
        Error::Lua(e)=>write!(f,"{}",e),
        Error::SQLite(e)=>write!(f,"{}",e),
        Error::Normal(e)=>write!(f,"{}",e),
        Error::Utf8Error(e)=>write!(f,"{}",e),
        Error::Mustache(e)=>write!(f,"mustache {}",e),
    };
    Ok(())
 }
}

impl<'de> Deserialize<'de> for Error {
  
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>{
          return  Ok(Error::OK);
        }
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
impl std::convert::From<std::str::Utf8Error> for Error{
    
fn from(e: std::str::Utf8Error) -> Self { 
    Error::Utf8Error(e)
 }
}
impl std::convert::From<serde_json::Error> for Error {
    fn from(_data: serde_json::Error) -> Self {
        Error::JSON(_data)
    }
}

impl std::convert::From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::SQLite(err)
    }
}

impl std::convert::From<mustache::Error> for Error {
    fn from(err: mustache::Error) -> Self {
        Error::Mustache(err)
    }
}