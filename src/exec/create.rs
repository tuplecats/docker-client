
use serde::{Deserialize, Serialize};

pub struct RequestBuilder {

    id: String,

    cmd: Vec<String>,

    user: String,

    attach_stderr: bool,

    attach_stdin: bool,

    attach_stdout: bool,

}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            id: "".to_string(),
            cmd: vec![],
            user: "".to_string(),
            attach_stderr: false,
            attach_stdin: false,
            attach_stdout: false
        }
    }
}

impl RequestBuilder {

    pub fn with_container<T>(container: T) -> Self
        where T: Into<String>
    {
        let mut builder = RequestBuilder::default();
        builder.id = container.into();
        builder
    }

    pub fn add_command<T>(&mut self, command: T) -> &mut Self
        where T: Into<String>
    {
        self.cmd.push(command.into());

        self
    }

    pub fn add_commands(&mut self, mut commands: Vec<String>) -> &mut Self {
        self.cmd.append(&mut commands);

        self
    }

    pub fn user<T>(&mut self, user: T) -> &mut Self
        where T: Into<String>
    {
        self.user = user.into();

        self
    }
    
    pub fn attach_stdin(&mut self, v: bool) -> &mut Self {
        self.attach_stdin = v;
        
        self
    }

    pub fn attach_stdout(&mut self, v: bool) -> &mut Self {
        self.attach_stdout = v;

        self
    }

    pub fn attach_stderr(&mut self, v: bool) -> &mut Self {
        self.attach_stderr = v;

        self
    }

    pub fn build(&self) -> Request {
        Request {
            id: self.id.clone(),
            cmd: self.cmd.clone(),
            user: self.user.clone(),
            attach_stdin: self.attach_stdin,
            attach_stderr: self.attach_stderr,
            attach_stdout: self.attach_stdout
        }
    }

}

#[derive(Deserialize, Serialize)]
pub struct Request {

    #[serde(skip_serializing)]
    id: String,

    #[serde(rename = "Cmd")]
    cmd: Vec<String>,

    #[serde(rename = "User", skip_serializing_if = "String::is_empty")]
    user: String,

    #[serde(rename = "AttachStderr")]
    attach_stderr: bool,

    #[serde(rename = "AttachStdin")]
    attach_stdin: bool,

    #[serde(rename = "AttachOut")]
    attach_stdout: bool,

}

impl Request {

    pub fn with_container<T>(container: T) -> RequestBuilder
        where T: Into<String>
    {
        RequestBuilder::with_container(container.into())
    }

    pub fn get_path(&self) -> String {
        format!("/containers/{}/exec", &self.id)
    }

}

#[derive(Deserialize, Serialize)]
pub struct Exec {

    #[serde(rename = "Id")]
    pub id: String

}