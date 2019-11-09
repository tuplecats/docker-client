use std::io::Read;

use unix_socket::UnixStream;

#[derive(Clone, Debug)]
pub struct Response {
    pub status: i32,
    pub body: String,
    pub content_length: usize,
}

impl From<String> for Response {
    fn from(text: String) -> Self {
        let components: Vec<&str> = text.split("\r\n\r\n").collect();

        if components.len() != 2 {
            panic!("Docker return invalid type");
        }

        let header = components[0];
        let body = components[1];

        let headers: Vec<&str> = header.split("\r\n").collect();

        let mut content_length = 0;
        for head in &headers {
            if head.contains("Content-Length:") {
                content_length = head.split(":").collect::<Vec<&str>>()[1].trim().parse().unwrap();
            }
        }

        let status = headers[0].split(" ").collect::<Vec<&str>>()[1];
        let status_code: i32 = status.parse().unwrap();

        Response {
            status: status_code,
            body: body.to_string(),
            content_length,
        }
    }
}

impl Response {
    pub fn read(stream: &mut UnixStream) -> Response {
        let result = &mut [0 as u8; 1024];

        let bytes = stream.read(result).unwrap();
        let mut resp = Response::from(std::str::from_utf8(result[0..bytes].as_ref()).unwrap().to_string());

        match resp.status {
            204 => {
                resp
            }
            _ => {
                let mut current_len = resp.body.len();
                let body = &mut [0 as u8; 1024];
                while current_len < resp.content_length {
                    let bytes = stream.read(body).unwrap();
                    current_len += bytes;
                    resp.body.push_str(std::str::from_utf8(body[0..bytes].as_ref()).unwrap());
                }

                resp
            }
        }
    }
}
