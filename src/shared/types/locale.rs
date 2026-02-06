pub enum Locale {
    EN,
    TH,
}

impl Locale {
    pub fn to_str(&self) -> &str {
        match self {
            Locale::EN => "en",
            Locale::TH => "th",
        }
    }
}
