pub struct Entry {
    pub(crate) heading: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

impl Entry {
    pub fn to_line(&self) -> String {
        format!("{}:{}:{}", self.heading, self.email, self.password)
    }

    pub fn from_line(line: &str) -> Option<Entry> {
        let mut parts = line.splitn(3, ":");

        Some(Entry {
            heading: parts.next()?.to_string(),
            email: parts.next()?.to_string(),
            password: parts.next()?.to_string(),
        })
    }
}