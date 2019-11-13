#[derive(Clone)]
pub struct DockerResponse {
    pub status: u16,
    pub body: String,
}