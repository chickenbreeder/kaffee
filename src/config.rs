/// Application configuration
#[derive(Debug)]
pub struct Config {
    pub title: String,
    pub width: u16,
    pub height: u16,
    pub resizable: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "kaffee".into(),
            width: 1024,
            height: 768,
            resizable: false,
        }
    }
}
