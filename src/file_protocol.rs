use csv::{ReaderBuilder, Terminator, Trim, Error as CsvError, Error};
use crate::etl_stream::{Category, EtlError, Indicator};

#[derive(Debug)]
pub struct File{
    pub name: String,
    pub path: String,
    pub category: Category,
}
#[derive(Debug)]
pub struct CsvParam {
    pub field_separator: u8,
    pub eol_marker: Terminator,
    pub has_header: bool,
}

impl File {
    pub fn load(&mut self) -> Result<&mut File, EtlError> {
        let full_path = format!("{}/{}", self.path, self.name);
        match &mut self.category {
            Category::Csv(csv_param) => {
                let mut rdr = match ReaderBuilder::new()
                    .delimiter(csv_param.field_separator)
                    .terminator(csv_param.eol_marker)
                    .has_headers(csv_param.has_header)
                    .trim(Trim::All)
                    .from_path(full_path.clone()) {
                    Ok(rdr) => rdr,
                    Err(e) => return Err(EtlError::IOError { name: full_path }),
                };

                let headers = rdr.headers();
                match headers {
                    Ok(h) => {
                        println!("Headers: {:?}", h);
                    },
                    Err(e) => {
                        return Err(EtlError::ParsingError { name: full_path, msg: format!("CSV error: {}", e) });
                    }
                }

                for rec in rdr.records() {
                    match rec {
                        Ok(rec) => {
                           println!("{:?}", rec);
                        },
                        Err(e) => {
                            return Err(EtlError::ParsingError { name: full_path, msg: format!("CSV error: {}", e) });
                        }
                    };
                }
                return Ok(self)
            },
            Category::Delimited=> Err(EtlError::NotSupported{name: "Delimited ".to_string()}),
            Category::Json =>  Err(EtlError::NotSupported{name: "Ftp".to_string()}),
        }
    }
}