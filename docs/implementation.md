# In this file I basically explain my decision making

## App design:
Here I'm **trying** to follow [The Elm Architecture](https://guide.elm-lang.org/architecture/),
implementing the model in `model/mod.rs`, the `update` functionality as a method
of the `Model` struct and the **drawing** in the `ui` module.

#### Messages:
The messages are defined in the `Message` enum, in the `events/messages.rs` module,
and are _polled_ by `events.poll_message`.

This function _polles a message_ polling events in the following order:
1. Input events (like mouse clicks, and key presses)
2. System events (like incoming events from a ssh connection)

## App flow
The app runs in the following way:
1. It first setup the terminal
2. Then it creates a Model with the default values (calling ::default())
3. A first frame is drawn using this default value
4. The main loop begins, and on each iteration the app polls a message
5. If there's a message available, the app calls the `model.update` method, passing
the current message and...
6. [...] finally, it calls `ui.draw` to update the current frame

## Sections
The sections are the different areas displayed, each with it's own separate task
