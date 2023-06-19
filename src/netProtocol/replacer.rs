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