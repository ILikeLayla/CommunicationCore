pub struct User {
    pub name: String
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User< name:{} >", self.name)
    }
}