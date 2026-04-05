# Technical requirements

- The game is written in Rust.
- The game engine chosen is Bevy.
- The target build is web.

# Goals

1. Create an arena with two walls and a divider
2. Add a paddle on either end of the play field. Use player inputs to move the paddles up and down.
3. Add a ball that moves around the arena and bounces off of paddles and walls.
4. Detect when the ball is leaves the arena. Assign a point to the player that scored.
5. Track and display the score of each player.

# Stretch Goals

1. Add a menu and allow the player to reset the game.
2. Add some basic sounds. Play a sound every time the ball collides with something, and every time the player scores.
3. Write an AI script that can follow the ball so you can play with only one player. Hint: Following the ball with a paddle is easy, but it makes the opponent impossible to beat. You might want to make the AI less than perfect somehow.

