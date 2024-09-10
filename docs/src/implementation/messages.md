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
All messages (and thus, events) are handled, asynchronously, by an instance of
`events::EventHandler`, which starts the `event loop` and populates the handler's
messages queue. This queue can then be consumed calling
`EventHandler::next_message()` in the main loop.
