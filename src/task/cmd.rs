use crate::{Complex, EnvVar, Input, Output};
use std::process::Command;
use std::sync::Arc;
use async_trait::async_trait;

use crate::task::Content;

/// [`CommandAction`] is a specific implementation of [`Complex`], used to execute operating system commands.
pub struct CommandAction {
    command: String,
}

impl CommandAction {
    #[allow(unused)]
    pub fn new(cmd: &str) -> Self {
        Self {
            command: cmd.to_owned(),
        }
    }
}

#[async_trait]
impl Complex for CommandAction {
    async fn run(&self, input: Input, _env: Arc<EnvVar>) -> Output {
        let mut args = Vec::new();
        let mut cmd = if cfg!(target_os = "windows") {
            args.push("-Command");
            Command::new("powershell")
        } else {
            args.push("-c");
            Command::new("sh")
        };
        args.push(&self.command);

        input.get_iter().for_each(|input| {
            if let Some(inp) = input.get::<String>() {
                args.push(inp)
            }
        });

        log::debug!("cmd: {:?}, args: {:?}", cmd.get_program(), args);
        let (code, out) = match cmd.args(args).output() {
            Ok(o) => (0, o),
            Err(e) => {
                return Output::error_with_exit_code(
                    e.raw_os_error(),
                    Some(Content::new(e.to_string())),
                )
            }
        };
        let stdout: Vec<String> = {
            let out = String::from_utf8(out.stdout).unwrap_or("".to_string());
            if cfg!(target_os = "windows") {
                out.rsplit_terminator("\r\n").map(str::to_string).collect()
            } else {
                out.split_terminator('\n').map(str::to_string).collect()
            }
        };
        let stderr: Vec<String> = {
            let out = String::from_utf8(out.stderr).unwrap_or("".to_string());
            if cfg!(target_os = "windows") {
                out.rsplit_terminator("\r\n").map(str::to_string).collect()
            } else {
                out.split_terminator('\n').map(str::to_string).collect()
            }
        };
        let output = Content::new((stdout, stderr));
        if out.status.success() {
            Output::new(output)
        } else {
            Output::error_with_exit_code(Some(code), Some(output))
        }
    }
}
