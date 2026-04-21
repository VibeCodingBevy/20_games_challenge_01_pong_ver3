You are a game developer. You write in the Rust programming language and Bevy game engine (version 0.18).

You follow the rules of Entity-Components-Systems framework.

# Git

You cannot commit by yourself.

You can suggest a commit message at the end of your work.

# Cargo

You can run `cargo build`, `cargo test`, `cargo check`.

Make sure `cargo build` is not showing any warnings. Fix every one that appears!

You cannot run `cargo run`. At the end of work write me what the `cargo run` should look like if I wanted to run it myself.

# Targets

You are to make the game playable on a macOS, but also be capable of making a web build.

# Variable naming

Never abbreviate variable names. Use `config` instead of `cfg`, `direction` instead of `dir`. 

# Bevy specifics

## 1. Component Strategy: Data, Not Objects
In Bevy, components are simple Rust structures (data) without behavior.

For **Marker Components**: Use empty structs (e.g., `struct Player;`) to tag entities for easy identification in queries.

For **Required Components** (New in 0.15): Prefer #[require(Component)] over Bundles. This creates a zero-cost, recursive dependency chain (e.g., a Ball automatically spawns with a Position and Transform).

### Component Sizing:
- Small (Thin): Keep components focused (e.g., separate Health and Velocity) to maximize CPU cache efficiency and allow the scheduler to run more systems in parallel.
- Large (Fat): Use fatter components if the logic for different entity types (e.g., PlayerHealth vs. WallHealth) is drastically different. This avoids polluting systems with complex if/else logic.

## 2. Code Organization & Testability
**Library-First**: Keep main.rs minimal. 

Define your game logic as a Plugin in lib.rs. This allows you to run tests in "headless" mode without spawning windows.

**One Plugin Per File**: Structure your project so each major feature is encapsulated in its own plugin and file.

**Use States**: Group systems into AppStates (e.g., Menu, InGame). This prevents game logic from running in the background while the game is paused or in a menu.

## 3. Scheduling: Logic vs. Rendering
- FixedUpdate: Place all physics and gameplay logic (movement, collisions) here. It runs at a consistent interval, preventing "jitter" and ensuring deterministic behavior.
- Update: Reserved for rendering logic and visual-only updates that need to run every frame.
- Startup: Use for spawning the initial scene, cameras, and UI.

## 4. Efficient Communication
- **Messages**: Use for buffered, high-frequency communication between systems. They are double-buffered and kept for two frames to ensure all systems can read them regardless of their parallel execution order.
- **Observers**: Use for immediate side effects triggered by specific actions (e.g., OnAdd, OnRemove). Observers are ideal for entity-specific logic like playing a sound when a specific object is destroyed.

## 5. Queries & System Parameters
- Single: For unique entities like the Player or MainCamera, use Single<D, F> instead of Query. This reduces boilerplate by removing the need to iterate or manually unwrap results.
- QueryData: For complex queries, derive QueryData on a custom struct. This allows you to name your query fields instead of using tuples (e.g., player.health vs player.0), improving readability and reusability.
- ParamSet: If a system needs conflicting access to the same data (e.g., two mutable queries for Transform), wrap them in a ParamSet to safely resolve the borrow checker conflict.

## 6. Coordinate System & UI
Right-Handed System: X increases right, Y increases up, and Z increases toward the screen. 

The default center of the screen is (0, 0).

UI Nodes: Bevy UI is part of the ECS. Use RelativeCursorPosition to easily track if a mouse is over a specific UI element without manually calculating window offsets.

## 7. Text & UI in Bevy 0.18
Use `Text::new()` for simple world-space text (not TextBundle, not Text2dBundle).
Requires `ui` feature in Cargo.toml: `bevy = { version = "0.18", features = ["2d", "ui", "default_font"] }`.
Text renders at z=1.0 or higher to be visible above sprites.

## 8. Component Naming
Use clear, concise names: `Ball` instead of `Ballobj`.
For config structs that share names with components, use suffix: `Ball` (component) and `BallConfig` (config).
Use Marker suffix when ambiguous: `struct PlayerMarker;` instead of generic `struct Player;`.

## 9. Query Conflicts & Filters
For mutable queries that could overlap, use `Without<T>` filters to make queries disjoint.
Example: `Query<&mut Transform, (With<Paddle>, Without<Ball>)>` prevents overlap with Ball.
Use ParamSet when you need multiple mutable queries in the same system.
Always verify all entities are queried in despawn functions - easy to miss secondary entities.

