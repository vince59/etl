mod etl_stream;
mod file_protocol;

use csv::Terminator;
use etl_stream::*;
use crate::file_protocol::{CsvParam, File};

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), Box<dyn std::error::Error>> {
    let my_file_name = "test1.csv".to_string();
    let my_file_path = "./sample_data/".to_string();

    let my_csv_param = CsvParam {
        field_separator: b';',
        eol_marker: Terminator::CRLF,
        has_header: true,
    };

    let my_category = Category::Csv(my_csv_param);

    let my_file = File {
        name: my_file_name.clone(),
        path: my_file_path.clone(),
        category: my_category,
    };



    let my_csv_file = Protocol::File(my_file);
    let mut my_data_source = DataSource::new(my_csv_file);

    my_data_source.load()?;
    
    Ok(())

}
