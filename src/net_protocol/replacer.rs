use super::*;
use std::rc::Rc;
use std::str;

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

impl Replace for DateTime<Local> {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String> {
        if rawdata.starts_with("DateTime<Local>".as_bytes()) {
            Ok(Box::new(Local::now()))
        } else {
            Err("damaged rawdata.".to_string())
        }
    }

    fn to_rawdata(&self) -> Vec<u8> {
        "DateTime<Local>".as_bytes().to_vec()
    }
}
