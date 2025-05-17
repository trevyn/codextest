# codextest

This repository includes a small command-line RPG written in Rust.
You travel across several locations and battle monsters along the way.

## Running the game

Use Cargo to build and run the project from the repository root:
```bash
cargo run -- [--automatic]
```

The optional `--automatic` flag now guides your character through the entire
adventure automatically. A log of each step and action is printed until the
journey finishes, including a list of all game features pulled from
`GAME_FEATURES.md`.
