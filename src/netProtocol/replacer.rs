use super::data_class::{Message, MessagesList, User};
use std::rc::Rc;
use chrono::naive::NaiveTime;
use chrono::{Local, DateTime};
use std::str;

pub trait Replace {
    fn to_rawdata(&self) -> Vec<u8>;
    fn from_rawdata(rawdata: Vec<u8>) -> Self;
}

impl Replace for Vec<u8> {
    fn from_rawdata(rawdata: Vec<u8>) -> Self {
        rawdata
    }
    fn to_rawdata(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Replace for DateTime<Local> {
    fn from_rawdata(_rawdata: Vec<u8>) -> Self {
        Local::now()
    }
    fn to_rawdata(&self) -> Vec<u8> {
        format!("{:?}", self).as_bytes().to_vec()
    }
}
