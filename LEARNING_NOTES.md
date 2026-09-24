# Learning notes

Short explanations of concepts encountered while building Dark Horse. Add to these notes as the project owner learns new ideas.

## Project setup

### Cargo

Cargo is Rust's project tool. It reads `Cargo.toml`, builds the program, manages dependencies, and runs the executable.

### `Cargo.toml`

The project's manifest: it contains the package name, settings, and dependencies the program uses.

### `src/main.rs`

The Rust source file where this executable starts. Rust calls its entry function `main`.

### `Cargo.lock`

Records the exact dependency versions Cargo selected. For an application, keep this file in version control so builds use a repeatable dependency set.

### Dependency (crate)

A dependency is code written elsewhere that the project uses. In Rust, a published package is commonly called a crate. Adding `winit` to `Cargo.toml` tells Cargo that the project depends on that library; Cargo resolves and downloads its package sources when building.

### `cargo run`

Asks Cargo to compile the application if needed and then run it. The first build can take longer because dependencies need to be downloaded and compiled.

## Window prototype

### `winit`

A Rust library for creating windows and receiving operating-system events such as resize, keyboard, and pointer input. It handles window management, not HTML rendering or drawing the browser interface itself.

### Event loop

The application waits for events from the operating system and responds to them. For example, it may redraw after a resize or close the window after a close request. This is the basic pattern used by many graphical applications.

### `ApplicationHandler`

In `winit`, this trait describes methods the application provides so the event loop can notify it about lifecycle changes and window events. Our `App` implements it; `resumed` creates the window, and `window_event` handles events sent to that window.

### `Option<Window>`

`Option<T>` is a Rust type that represents either `Some(value)` or `None`. The app starts with no window (`None`) and stores the created window in `Some(window)`. Keeping the window in `App` keeps it alive while the application runs.

### Close request

The operating system sends `WindowEvent::CloseRequested` when the user asks to close a window. Calling `event_loop.exit()` tells `winit` that the application should stop its event loop.

## Build output

### `target/`

Cargo's generated build output directory. It contains compiled files and can be recreated from the source and manifest, so it is normally excluded from Git.
