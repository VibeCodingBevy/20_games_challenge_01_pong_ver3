# Technical requirements

- The game is written in Rust.
- The game engine chosen is Bevy.
- The target build is web.
- Each number is considered a parameter that can be changed, and the game does not require a recompile, just a restart. The values are stored in a file.

# Game Specification

## Screen
- Window size: 800x600
- Ball exits arena through left/right edges only

## Objects
- Ball: 20px diameter, 400 px/s
- Paddle: 100x30px (width x height), 300 px/s
- Wall: 10px thick
- Paddles: 20px margin from left/right edges, centered vertically

## Physics
- Ball speed: 400 px/s (fixed throughout rally)
- Paddle speed: 300 px/s
- Wall thickness: 10px
- Bounce angle: Angle based on where ball hits paddle (hit top = bounce up, hit bottom = bounce down)

## Controls
- Left paddle: Arrow Up / Arrow Down
- Right paddle: Arrow Up / Arrow Down

## Scoring
- Endless play (no winning score)
- After score: ball resets at center, direction toward the player who lost the point

## Config File
All parameters stored in `config.toml`. Example structure:

```toml
[screen]
width = 800
height = 600

[ball]
diameter = 20
speed = 400

[paddle]
width = 100
height = 30
margin = 20
speed = 300

[arena]
wall_thickness = 10
```

# Goals

## 1. Create an arena with top and bottom walls and a center line
**Acceptance Criteria:**
- Top wall: visible at y=0, full width, 10px thick
- Bottom wall: visible at y=590, full width, 10px thick
- Center line: visible at x=400, runs full height

## 2. Add paddles with player inputs
**Acceptance Criteria:**
- Left paddle: positioned at x=20 (margin), vertically centered
- Right paddle: positioned at x=780 (800-margin-paddle_width), vertically centered
- Both paddles respond to Arrow Up/Down
- Paddles stop at top/bottom wall boundaries

## 3. Add ball with physics
**Acceptance Criteria:**
- Ball spawns at center (400, 300)
- Ball travels at exactly 400 px/s
- Ball bounces off top wall: Y velocity reverses
- Ball bounces off bottom wall: Y velocity reverses
- Ball bounces off paddle: angle varies based on hit position (top = up, bottom = down)

## 4. Detect ball exits and assign points
**Acceptance Criteria:**
- Ball x < 0: right player scores
- Ball x > 800: left player scores
- Ball resets to center, moves toward player who lost the point
- Score increments by 1

## 5. Display scores
**Acceptance Criteria:**
- Both scores visible on screen at all times
- Scores update immediately after a point
- Starting scores: 0 - 0
