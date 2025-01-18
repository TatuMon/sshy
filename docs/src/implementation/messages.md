# Messages
The messages are the most important piece of data in this architecture. They are
events that define the communication between the **user** and the **model
(state)** of the application.

As an example, when the user presses `q`, a message of type
`Message::ShowPopup(Popup::ExitPrompt)` is sent to the model, indicating that
it should update it's state to show the `ExitPrompt` popup

> The messages are defined in the `Message` enum, in the `events/messages.rs`
module

## Message and event handling
All events, like user input and [commands/subprocesses](./commands.md), are
handled by the EventHandler. This struct has a public method called `poll_messages`,
that polles events from different sources, creates the appropiate messages based
on the events and the current state of the model, and returns an iterator over
a queue of messages, which are then processed by the main loop, thus updating
the model's state.

> Commands and subprocesses event handling is explained [here](./commands.md#task-event-handling)
