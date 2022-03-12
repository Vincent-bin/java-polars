use polars::prelude::*;

pub struct JDataFrame{
    pub df: DataFrame,
}

impl JDataFrame{
    pub fn print(&self) -> String{
        format!("{:?}", self.df)
    }
}

