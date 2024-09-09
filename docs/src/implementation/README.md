# Implementation:
I'm **trying** to follow [The Elm Architecture](https://guide.elm-lang.org/architecture/),
implementing the model in `model/mod.rs`, the `update` functionality as a method
of the `Model` struct and the **drawing** in the `ui` module.

## App flow
The app runs in the following way:
1. It first setup the terminal
2. Then it creates a Model with the default values (calling ::default())
3. The first frame is drawn using this default value
4. The main loop begins, and on each iteration the app polls a [message](./messages.md)
5. If there's a message available, the app calls the `model.update` method, passing
the current message and...
6. [...] finally, it calls `ui.draw` to update the current frame
