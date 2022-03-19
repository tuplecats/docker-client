

#[derive(Default)]
pub struct RequestBuilder {

    from_image: String,

    from_src: String,

    repo: String,

    tag: String,

    message: String,

    platform: String

}

impl RequestBuilder {

    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn image<T>(mut self, image: T) -> Self
        where T: Into<String>
    {
        self.from_image = image.into();

        self
    }

    pub fn source<T>(mut self, source: T) -> Self
        where T: Into<String>
    {
        self.from_src = source.into();

        self
    }

    pub fn repo<T>(mut self, repo: T) -> Self
        where T: Into<String>
    {
        self.repo = repo.into();

        self
    }

    pub fn tag<T>(mut self, tag: T) -> Self
        where T: Into<String>
    {
        self.tag = tag.into();

        self
    }

    pub fn message<T>(mut self, message: T) -> Self
        where T: Into<String>
    {
        self.message = message.into();

        self
    }

    pub fn platform<T>(mut self, platform: T) -> Self
        where T: Into<String>
    {
        self.platform = platform.into();

        self
    }

    pub fn build(self) -> Request {
        Request {
            from_image: self.from_image,
            from_src: self.from_src,
            repo: self.repo,
            tag: match self.tag.as_str() {
                "" => String::from("latest"),
                _=> self.tag.clone()
            },
            message: self.message,
            platform: self.platform
        }
    }

}

pub struct Request {

    from_image: String,

    from_src: String,

    repo: String,

    tag: String,

    message: String,

    platform: String

}

impl Request {

    pub fn image(&self) -> &str {
        &self.from_image
    }

    pub fn source(&self) -> &str {
        &self.from_src
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn platform(&self) -> &str {
        &self.platform
    }

    pub fn get_path(&self) -> String {

        let mut path = String::from("/images/create?");

        if !self.from_image.is_empty() {
            path.push_str(format!("{}={}&", "fromImage", self.from_image).as_str());
        }

        if !self.from_src.is_empty() {
            path.push_str(format!("{}={}&", "fromSrc", self.from_src).as_str());
        }

        if !self.repo.is_empty() {
            path.push_str(format!("{}={}&", "repo", self.repo).as_str());
        }

        if !self.tag.is_empty() {
            path.push_str(format!("{}={}&", "tag", self.tag).as_str());
        }

        if !self.message.is_empty() {
            path.push_str(format!("{}={}&", "message", self.message).as_str());
        }

        if !self.platform.is_empty() {
            path.push_str(format!("{}={}&", "platform", self.platform).as_str());
        }

        path.pop();
        path
    }

}