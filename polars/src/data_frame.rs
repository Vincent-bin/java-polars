use polars::prelude::*;
use std::fs::File;

pub struct JDataFrame {
    pub df: DataFrame,
}

impl JDataFrame {
    pub fn print(&self) -> String {
        format!("{:?}", self.df)
    }
}

pub fn read_csv(file_path: String) -> Result<JDataFrame> {
    let p = std::path::Path::new(&file_path);
    let p = resolve_homedir(p);
    let f = File::open(&p)?;
    let df = CsvReader::new(f).finish().unwrap();
    Ok(JDataFrame { df })
}
