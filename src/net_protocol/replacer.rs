use std::str;
use chrono::{DateTime, Local};

pub trait Outputer {
    fn output<'a, T>(&self, start: &'a str, formatter: fn(&Self) -> T, err: &str) -> Result<Box<T>, String>;
}

impl Outputer for Vec<u8> {
    fn output<'a, T>(&self, start: &'a str, formatter: fn(&Self) -> T, err: &str) -> Result<Box<T>, String> {
        if self.starts_with(start.as_bytes()) {
            Ok(Box::new(formatter(self)))
        } else {
            Err(err.to_string())
        }
    }
}

pub trait Replace {
    fn to_rawdata(&self) -> Vec<u8>;
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String>;
}

impl Replace for Vec<u8> {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Vec<u8>>, String> {
        Ok(Box::new(rawdata))
    }
    fn to_rawdata(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Replace for String {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String> {
        if let Ok(data) = str::from_utf8(&rawdata) {
            Ok(Box::new(data.to_string()))
        } else {
            Err("#Dfr02".to_string())
        }
    }
    fn to_rawdata(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Replace for DateTime<Local> {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String> {
        rawdata.output::<DateTime<Local>>("DateTime<Local>", |_| {
            Local::now()
        }, "#Dfr01")
    }

    fn to_rawdata(&self) -> Vec<u8> {
        "DateTime<Local>".as_bytes().to_vec()
    }
}
