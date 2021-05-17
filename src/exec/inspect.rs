
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ExecStatus {

    #[serde(rename = "CanRemove")]
    can_remove: bool,

    #[serde(rename = "DetachKeys")]
    detach_keys: String,

    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "Running")]
    pub running: bool,

    #[serde(rename = "ExitCode")]
    pub exit_code: i32,

    #[serde(rename = "OpenStdin")]
    open_stdin: bool,

    #[serde(rename = "OpenStderr")]
    open_stderr: bool,

    #[serde(rename = "OpenStdout")]
    open_stdout: bool,

    #[serde(rename = "ContainerID")]
    container_id: String,

    #[serde(rename = "Pid")]
    pid: i32

}