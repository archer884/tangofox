use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Address {
    port: u16,
    external: bool,
}

impl Address {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            external: true,
        }
    }
}

impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.to_string()
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.external {
            write!(f, "0.0.0.0:{}", self.port)
        } else {
            write!(f, "127.0.0.1:{}", self.port)
        }
    }
}
