pub struct Request {
    pub method: &'static str,
    pub path: String,
    pub host: String,
}

impl Request {
    pub fn get(host: String, path: String) -> Self {
        Self {
            method: "GET",
            path,
            host,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(256);

        buffer.extend_from_slice(self.method.as_bytes());
        buffer.extend_from_slice(b" ");
        buffer.extend_from_slice(self.path.as_bytes());
        buffer.extend_from_slice(b" HTTP/1.1\r\n");

        buffer.extend_from_slice(b"Host: ");
        buffer.extend_from_slice(self.host.as_bytes());
        buffer.extend_from_slice(b"\r\n");

        buffer.extend_from_slice(b"Connection: close\r\n");
        buffer.extend_from_slice(b"\r\n");

        buffer
    }
}