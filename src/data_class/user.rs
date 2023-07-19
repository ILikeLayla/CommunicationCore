use super::Outputer;

#[derive(PartialEq, Eq, Hash)]
pub struct User {
    pub name: String
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User<{}>", self.name)
    }
}

impl super::Replace for User {
    fn from_rawdata(rawdata: Vec<u8>) -> Result<Box<Self>, String> {
        rawdata.output::<User>("User", |data| {
            let data = &data[5..(data.len()-1)];
            User::new(std::str::from_utf8(&data).unwrap())
        }, "#Dfr03")
    }
    fn to_rawdata(&self) -> Vec<u8> {
        format!("{}", self).as_bytes().to_vec()
    }
}