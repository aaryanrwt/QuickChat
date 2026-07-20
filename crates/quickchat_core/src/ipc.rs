use std::io;

#[derive(Debug)]
pub enum EditorTarget {
    VSCode,
    Neovim,
    IntelliJ,
}

/// A command sent over the IPC bridge to control the external editor.
#[derive(Debug)]
pub enum EditorCommand {
    /// Jump to a specific file and line number
    GoToLine { file_path: String, line_number: u32 },
}

pub struct IpcBridge {
    // In a real implementation, this would hold the connection to the editor's socket/pipe.
    // target: EditorTarget,
    // socket_path: String,
}

impl IpcBridge {
    pub fn new(_target: EditorTarget) -> Self {
        Self {}
    }

    /// Sends a JSON-RPC command to the editor to open the file.
    pub async fn send_command(&self, cmd: EditorCommand) -> io::Result<()> {
        match cmd {
            EditorCommand::GoToLine {
                file_path,
                line_number,
            } => {
                println!(
                    "IPC [MOCK]: Telling editor to open {} at line {}",
                    file_path, line_number
                );
                // Implementation: Serialize to JSON and write to socket
            }
        }
        Ok(())
    }
}
