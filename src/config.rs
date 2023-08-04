pub struct Config {
    pub fps: i8,
}

impl Config {
    pub fn new() -> Self {
        Config {
            fps: 10,
        }
    }
}
