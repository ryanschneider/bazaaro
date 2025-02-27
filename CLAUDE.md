# Bazaaro Development Guidelines

## Build Commands
- Build: `cargo build`
- Run: `cargo run`
- Release build: `cargo build --release`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Run specific test: `cargo test test_name`

## Code Style
- **Imports**: Use `use bevy::prelude::*` for Bevy imports, organize other imports alphabetically
- **Formatting**: Follow standard Rust style with 4-space indentation
- **Types**: Use strong typing with Components, Resources, and Events
- **Naming**:
  - Components/Structs: PascalCase (e.g., `Character`, `Health`)
  - Variables/fields: snake_case (e.g., `current`, `max`)
  - Systems: snake_case, descriptive of action (e.g., `update_health`, `spawn_characters`)
- **Error handling**: Use `Result` when dealing with fallible operations
- **Documentation**: Document public APIs, especially non-obvious behavior
- **Component structure**: Use ECS approach, prefer composition over inheritance
- **Game state**: Use `States` for managing different game phases (Loading, Fight, Results)