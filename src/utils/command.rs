use std::process::Command;

// Struct to handle child commands asynchronously
//
// Commands should be ran using the "run" method, which instantiate a
// std::process::Command instance. This also creates a channel to handle
// communication with the stdin and stdout; that can be done using the "read"
// method to get data from stdout and stderr, and "write" to write data to stdin
// pub struct SshyCommand {
//     cmd: Option<Command>
// }

/// Struct to handle child command synchronously
pub struct SyncCommand {
    cmd: Option<Command>,
}
