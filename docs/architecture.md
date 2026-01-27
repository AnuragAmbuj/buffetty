# Architecture Documentation

This document explains the high-level architecture of `buffetty` (Alacritty fork) to aid in understanding the codebase and recent changes for GTK support.

## Core Components

The workspace consists of several crates:

### 1. `alacritty_terminal`

This is the core logic library. It is completely independent of the windowing system or renderer.

- **`Term`**: The main state machine. It handles ANSI escape sequences, manages the grid of cells, and tracks cursor position.
- **`Grid`**: A 2D array of `Cell`s representing the terminal's visible content and scrollback buffer.
- **`Event`**: Defines abstract terminal events (e.g., input, resizing) that the frontend must handle.

### 2. `alacritty` (The Frontend)

This crate provides the windowing, input handling, and rendering.

- **`renderer`**: OpenGL renderer. It consumes the `Grid` from `alacritty_terminal` and draws it to the screen using shaders.
- **`display`**: Orchestrates the window creation (using `winit`) and the renderer. It manages the `EventLoop`.
- **`event`**: Input processing. Converts `winit` events (keyboard, mouse) into `alacritty_terminal` actions.
- **`config`**: Handles loading and merging of configuration files (TOML).

### 3. `alacritty_config`

A helper crate for defining configuration structures and parsing logic, shared by other crates.

## Execution Flow (Standard)

1.  **Startup**: `main.rs` initializes the logger, config, and `Display`.
2.  **Event Loop**: `winit::EventLoop` runs (in `processor.run`).
3.  **Input**: Key presses are captured by `winit`, translated by `input.rs`, and sent to `Term::write`.
4.  **Update**: `Term` processes the input, updating the `Grid`.
5.  **Render**: `Display::draw` is called. It locks the `Term`, iterates over `RenderableContent` (visible cells), and submits draw calls to the GPU via `Renderer`.

## GTK Integration (Proposed)

To support GTK tabs, we introduce a new frontend structure:

1.  **`alacritty` as a Library**: The `renderer`, `display`, and `event` modules are exposed so they can be reused.
2.  **`alacritty_gtk`**: A new binary crate.
    - **Windowing**: Uses `gtk4::ApplicationWindow` instead of `winit`.
    - **Tabs**: Uses `gtk4::Notebook` to manage multiple terminal instances.
    - **Rendering**: Uses `gtk4::GLArea` to provide an OpenGL context. It reuses `alacritty::renderer::Renderer` to draw the terminal content inside the GL widget.
    - **Input**: Converts GDK events into `alacritty_terminal` actions.
