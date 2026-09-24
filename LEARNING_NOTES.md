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

### `egui` and `eframe`

`egui` provides interface widgets such as labels, buttons, and editable text fields. It uses an immediate-mode approach: application code describes the interface during each UI update, and egui reports interactions such as text edits or button clicks. `eframe` is the app framework that connects egui to a native window, input events, and a renderer on desktop platforms. We use these for the browser's own controls; they are not a web page rendering engine.

### `String` and `&mut`

The address is stored in a `String`, which can hold text that changes while the program runs. `&mut self.address` passes the text field temporary permission to change that value. The app still owns the string and can use its updated value in later UI updates.

### `TextEdit::singleline`

This egui widget provides one editable line of text, including normal cursor movement, selection, and text input. In v0.0.2 it only collects an address; it does not start a network request.

### `CentralPanel`

An egui panel is a layout area for arranging widgets. `CentralPanel` fills the main content area of the app window; its closure describes the controls shown there.

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

## Git workflow

### Branch

A branch is a named line of work. We create a branch for each milestone, such as `feature/v0.0.1-window`, so its changes can be developed and reviewed separately from `main`.

### Commit

A commit is a saved snapshot of the staged changes. A useful commit has a short message describing what changed.

### Push and pull

`git push` sends local commits to a remote repository such as GitHub. `git pull` brings commits from the remote branch into the local branch.

### Pull request and merge

A pull request (PR) proposes merging one branch into another on GitHub. Reviewing and merging the PR brings the milestone changes into `main`.

### Milestone workflow used here

Create a milestone branch, make and commit the change, push the branch, open a PR into `main`, merge it, then switch back to `main` and pull. The project owner is practicing this workflow one milestone at a time.
