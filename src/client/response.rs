use hyper::body::Bytes;
use std::path::Path;

#[derive(Clone)]
pub struct DockerResponse {
    pub status: u16,
    pub body: Bytes,
}

impl DockerResponse {

    pub fn body_as_string(&self) -> String {
        String::from_utf8(
        self.body.to_vec()
        ).unwrap()
    }

    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, self.body.to_vec())
    }

}