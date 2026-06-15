# Implement Configurable Winning Score

## Changes

### 1. `pong/config.toml`
Add `[game]` section at end of file:
```toml
[game]
winning_score = 10
```

### 2. `pong/src/components.rs`
Add `GameConfig` struct and include in `Config`:
- Change `Config` to include `pub game: GameConfig`
- Add new struct:
  ```rust
  #[derive(Deserialize)]
  pub struct GameConfig { pub winning_score: u32 }
  ```

### 3. `pong/src/game_plugin.rs`
- Remove line: `const WINNING_SCORE: u32 = 10;`
- In `handle_scoring_system`, change `WINNING_SCORE` to `config.game.winning_score`

### 4. `pong/src/game_over_plugin.rs`
- Add `use crate::components::Config;` import
- Add `config: Res<Config>` parameter to `show_game_over`
- Change `score.left >= 10` to `score.left >= config.game.winning_score`

## Verification
Run `cargo check` (or `cargo build`) to confirm no warnings/errors.
