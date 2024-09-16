# Commands and subprocesses

## Commands
These are jobs that perform only one task and are terminated as soon as they
finished, so only one can be ran at a time.

While these jobs are running, only the SigTerm [message](./messages.md) should
be handled, which sends a `SIGINT` signal to it.

At the same time, this message should only be available when a *command*
is running

Examples of **commands** are:
- ssh-keygen, used when adding a new key
- ssh-add, used when adding a new key to the ssh agent

## Subprocesses
These are jobs intended to run in the background of the main process.

> For simplicity, I will use "task" to refer to both commands and subprocesses

## Tasks abstractions
Each command and subprocess available in the app is abstracted via a struct
defined in the `commands` and `subprocesses` modules, respectively. These structs
will contain the command/process state and define all the available operations
(e.g.: `start` or `interrupt`).

## Tasks state
Remember that the model must be the `single source of truth` of the app, so the
list of running processes and their state must be defined there.

#### Example of a command's life-cycle
1. Having focus in the `AddPubKey` popup, the user, after specifying the new key's
name, type and comment, presses "enter"
2. This will cause the `EventHandler` to add a message of type
`CmdSpawned(commands::SshKeygen)`
3. Which tells...
    - the model to set it's current command to `Some(commands::SshKeygen)`
    - and the event handler to start the `ssh-keygen`, using the model's 
    `public_keys_list_state::NewPublicKeyState`, and binding the command's state
    with the event handler via a copy of it's `msg_tx`
4. Having the model's state updated, the UI now shows a loading indicator
5. At this point, the command is running, so only 2 things can happen:
    1. The user presses `CTRL+C`, triggering a `SigInt(commands::SshKeygen)`
    message. All other input is ignored.
    2. The command prompts the user for input, triggering a `CmdShowInputPrompt`
    message.
    3. The command finished, triggering a `CmdFinished(commands::SshKeygen)`
    message
6. If points 5.1 or 5.3 happen, the UI will show a message indicating which of the
two happened
7. On the other hand, if point 5.2 happened, the popup `CmdInputPrompt` is shown,
prompting the user for input that will be forwarded to `commands::SshKeygen`
8. Finally, when the command finishes (after receiving the `CmdFinished(...)`
message), a popup will show the result of it.

## Task event handling
All tasks should communicate with the main loop via messages, like every
communication in this app. To do this, the EventHandler has two fields defined,
`task_msg_rx` and `task_msg_tx`, which are the receiving and sending halves of
`mpsc` channel. All created commands and subprocesses must have a copy of
`task_msg_tx` to be able to communicate with the receiving end of the channel,
which is reserved for the EventHandler that then reads it when
`poll_messages` is called.
