use std::{error, fmt};
use serde_json::{Map, Value};
use crate::file_protocol::{CsvParam, File};
use csv::Error as CsvError;
use crate::table::Table;

#[derive(Clone, Copy, Debug)]
pub enum Encoding {
    Utf8,
    Ascii,
}

#[derive(Debug)]
pub struct CsvConfig {
    pub encoding: Encoding,
    pub eol_markers: Vec<String>,
    pub separators: Vec<String>,
}

#[derive(Debug)]
pub struct Indicator {
    pub name: String,
    pub level: usize
}


#[derive(Debug)]
pub enum Category {
    Csv(CsvParam),
    Delimited,
    Json,
}

#[derive(Debug)]
pub enum Protocol {
    File(File),
    Ftp,
    Http,
    Bdd,
    StdIO,
    Clipboard,
}

#[derive(Debug)]
pub enum EtlError {
    IOError{name: String},
    NotSupported{name: String},
    StdError{msg: String},
    ParsingError{name: String, msg: String},
}

impl fmt::Display for EtlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EtlError::IOError { name } => write!(f, "IO Error with {}", name),
            EtlError::NotSupported { name } => write!(f, "Protocol not supported {}",name),
            EtlError::StdError { msg } => write!(f, "{}", msg),
            EtlError::ParsingError { name, msg } => write!(f, "Parsing error with {} \n {}", name, msg),
        }
    }
}

impl error::Error for EtlError {}

pub struct DataSource {
    protocol: Protocol,
    data: Option<Table>,
}

impl DataSource {
    pub fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            data:None,
        }
    }
    
    pub fn load(&mut self) -> Result<&mut DataSource, EtlError> {
        match &mut self.protocol {
            Protocol::File(f) => {
                self.data=Some(f.load()?);
                println!("{:?}", self.data);
                return Ok(self)
            },
            Protocol::Ftp => Err(EtlError::NotSupported{name: "Ftp".to_string()}),
            Protocol::Http => Err(EtlError::NotSupported{name: "Http".to_string()}),
            Protocol::Bdd => Err(EtlError::NotSupported{name: "Bdd".to_string()}),
            Protocol::StdIO => Err(EtlError::NotSupported{name: "StdIO".to_string()}),
            Protocol::Clipboard => Err(EtlError::NotSupported{name: "Clipboard".to_string()})
        }
    }
}

pub struct Stream {
    input: DataSource,
    output: DataSource,
}

impl Stream {
    pub fn new(input: DataSource , output: DataSource ) -> Self {
        Self {
            input,
            output,
        }
    }
    pub fn load(&mut self) -> &mut Stream {
        self
    }
}