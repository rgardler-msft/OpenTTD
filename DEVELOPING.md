# Developing the Rust Port

This file describes how to build and run the Rust-based migration tooling on a new machine, and how to see the current windowing scaffold running.

## Quick start (stub backend, no system SDL2 needed)

1) Ensure Rust is installed (rustup recommended): https://rustup.rs
2) Load the Rust environment in your shell:

```
source "$HOME/.cargo/env"
```

3) Run the CLI with the stub window (no real window, for testing):

```
cd rust
cargo run --bin openttd_cli -- --window
```

You should see `window created` printed to stdout. Press Ctrl+C to exit.

## Run with SDL2 window (real window)

Install SDL2 development libraries, then run with the SDL2 feature enabled.

### Install SDL2 Dependencies

#### Debian/Ubuntu
```
sudo apt-get update
sudo apt-get install -y libsdl2-dev
```

#### macOS
```
brew install sdl2
```

#### Fedora
```
sudo dnf install SDL2-devel
```

### Run the SDL2 Window

There are two ways to run the application with SDL2:

#### Option 1: Run directly with cargo (recommended)
```
cd rust
cargo run --bin openttd_cli --features sdl2 -- --window
```

#### Option 2: Build and run the binary
```
cd rust
cargo build --release --features sdl2
./target/release/openttd_cli --window
```

#### Option 3: With a savegame file
```
cd rust
cargo run --bin openttd_cli --features sdl2 -- --window ../regression/regression/test.sav
```

### What to Expect

When you run with SDL2 enabled:
- A 640x480 window titled "OpenTTD" will appear
- The window will have a dark blue background (RGB: 24, 40, 72)
- The console will display:
  - `video driver: <driver_name>` (e.g., x11, wayland, windows)
  - `window created`
  - Event logs for window interactions (resize, focus, etc.)
- To exit: Close the window or press Ctrl+C in the terminal

### Troubleshooting

#### WSL (Windows Subsystem for Linux)

On WSL, you may see the window listed in Alt-Tab but not visible. In that case:

1. Exit the loop with Ctrl+C
2. Try forcing a specific video driver:
   ```
   SDL_VIDEODRIVER=x11 cargo run --bin openttd_cli --features sdl2 -- --window
   ```

#### SDL2 not found errors

If you get linking errors about SDL2:
- Ensure SDL2 development libraries are installed (see installation section above)
- On some systems, you may need to set `PKG_CONFIG_PATH`:
  ```
  export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH
  ```

#### Headless environments

The SDL2 tests will fail in headless environments (CI, SSH without X forwarding). Use the stub backend for testing in these environments.

## Tests

### Run all tests with stub backend (default)
```
cd rust
cargo test
```

### Run tests for a specific crate
```
cd rust
cargo test -p openttd_video
cargo test -p openttd_savegame
```

### Run with SDL2 feature (will fail in headless environments)
```
cd rust
cargo test --features sdl2
```

## Project Structure

- `rust/openttd_video/` - Video/windowing subsystem (SDL2 and stub backends)
  - `src/sdl2.rs` - SDL2 windowing implementation
- `rust/openttd_savegame/` - Savegame parsing functionality
- `rust/openttd_cli/` - CLI application entry point
  - `src/main.rs` - Main entry point with `--window` flag support
- `regression/` - Sample save files for testing

## Current Implementation Status

The Rust port currently includes:
- **Savegame parsing**: Can read and parse OpenTTD save file headers
- **Basic SDL2 windowing**: Window creation, event loop, fullscreen toggle
- **Stub backend**: For testing without SDL2 dependencies

See work items in the worklog (`wl list`) for ongoing development tasks.
