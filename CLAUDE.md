# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A castle defense game inspired by "Defend your Castle", built with Bevy 0.17.3 and targeting WASM as the primary platform. Players defend their castle by dragging attacking peasants and throwing them away.

## Build & Development Commands

### Running the Game
```bash
cargo run
```
The project is configured (`.cargo/config.toml`) to automatically build for `wasm32-unknown-unknown` and run with `wasm-server-runner`, which creates a local webserver.

### Building
```bash
cargo build                           # Default WASM build
cargo build --release                 # Optimized release build
```

### Testing
```bash
cargo test                            # Run all tests
cargo test -p <crate_name>           # Run tests for specific crate (castle, enemy, etc.)
```

### Linting & Formatting
```bash
cargo clippy                          # Run linter
cargo fmt                             # Format code
cargo fmt -- --check                  # Check formatting without modifying
cargo fix --allow-staged --allow-dirty  # Auto-fix lint issues
```

The pre-commit hook (`.husky/hooks/pre-commit`) automatically runs `cargo fix` and `cargo fmt` on commit.

### Prerequisites
- Rust nightly toolchain (specified in `rust-toolchain.toml`)
- WASM target: `rustup target add wasm32-unknown-unknown`
- wasm-server-runner: `cargo install wasm-server-runner`
- OS-specific Bevy dependencies (see [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/))
  - Linux: `libasound2-dev`, `libudev-dev`, `pkg-config`

## MCP Server Usage

This project benefits from several MCP servers available in the environment. Use these tools proactively:

**Git & GitHub:**
- Use the `git` MCP server for advanced git operations (viewing history, diffs, branch management)
- Use the `github` MCP server for creating/managing issues, pull requests, and viewing repository metadata
- Prefer MCP tools over bash commands for git operations when available

**Fetch:**
- Use the `fetch` MCP server to retrieve Bevy documentation, crates.io package info, or external resources
- Helpful for checking latest Bevy/Rapier API docs or finding solutions to common patterns

**Memory:**
- Use the `memory` MCP server to persist important project decisions, architecture notes, or frequently needed context
- Store information about why certain architectural choices were made
- Remember common issues and their solutions across sessions

**Context7:**
- Use for enhanced context management when working on complex, multi-file changes
- Helpful for tracking related changes across the workspace crates

## Architecture

### Workspace Structure

The project uses a Cargo workspace with 9 crates organized by domain responsibility:

**Core Application:**
- **survive_main**: Entry point that orchestrates all plugins and initializes the Bevy app

**Shared Data:**
- **models**: Core components and types used across all crates
  - `Health`, `Attack`, `Speed`, `Hardness`, `Distance`, `Name`
  - `Draggable`, `Dragged` - UI interaction markers
  - `GameState` enum - MainMenu, Playing, Paused, GameOver

**Game Systems (Plugins):**
- **castle**: Castle entity definition and spawning (300x150, Health: 20, Hardness: 10)
- **enemy**: Enemy types and movement (Peasant: 64x64, Health: 5, Speed: 100, draggable)
- **combat**: Combat mechanics with event-driven damage system
- **level**: Game setup, ground spawning, and enemy wave management (spawns every 5s)
- **camera**: Camera controls (follow, drag-to-pan, scroll-to-zoom, peasant dragging)
- **ui**: Menu systems (MainMenu, HUD, PauseMenu, GameOverMenu)
- **diagnostics**: Debug utilities (Bevy diagnostics, Rapier debug rendering)

### Dependency Flow
```
survive_main (orchestrator)
├── models (shared types - no internal dependencies)
├── castle (depends on: models)
├── enemy (depends on: models, castle)
├── combat (depends on: models, castle)
├── level (depends on: models, castle, enemy, combat)
├── camera (depends on: models, castle)
├── ui (depends on: models, castle)
└── diagnostics (standalone)
```

### Key Architectural Patterns

**Plugin-Based Architecture:**
Each domain exports a Bevy `Plugin` that registers its systems and components. The main app in `survive_main` composes these plugins.

**Event-Driven Combat:**
```
Collision/Range Detection → DamageEvent → Health Update → DeathEvent → Despawn/GameOver
```
- `combat::detect_attacks` - Checks range, emits DamageEvent
- `combat::handle_collisions` - Converts Rapier ContactForceEvent to DamageEvent based on hardness
- `combat::apply_damage` - Processes DamageEvent, updates Health, emits DeathEvent
- `combat::handle_deaths` - Despawns entities or triggers GameOver

**State Management:**
Systems run conditionally based on `GameState`:
- `OnEnter(GameState::Playing)` - Setup (spawn castle, ground)
- `Update.run_if(in_state(Playing))` - Active gameplay systems
- `OnExit(Playing).run_if(not(in_state(Paused)))` - Cleanup (conditional to preserve state when paused)

**ECS Component Composition:**
Entities are compositions of small, focused components. Use `#[require(...)]` for component dependencies.

**Physics Integration:**
Bevy Rapier2D (2D rigid bodies, 100 pixels per meter):
- Colliders on Castle, Ground, Peasants
- ContactForceEvent drives collision damage
- Kinematic controls for peasant dragging

### Game State Flow
```
MainMenu → Playing ↔ Paused → GameOver
```
- MainMenu: Initial state, "Start Game" button transitions to Playing
- Playing: Active gameplay
- Paused: Auto-triggered on window focus loss, freezes gameplay
- GameOver: Triggered when castle Health reaches 0

## Code Conventions

- Strong typing with newtype patterns for domain clarity (Health(u16), Distance(u16), Speed(f32))
- Use `derive_more` for ergonomic arithmetic operations on newtypes
- Each crate exports one main plugin (`CastlePlugin`, `EnemyPlugin`, etc.)
- Event-driven for decoupled systems (DamageEvent, DeathEvent)
- Query-based system parameters for efficient ECS access