# Tetris Game in Rust

This repository contains a simple Tetris game developed in Rust, utilizing the `piston_window` crate for graphics rendering.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Features
- Randomly seeded T, O, I, L, S tiles
- Tiles can be rotated and moved freely
- Game over if tiles reach the top
- Score by completing and removing filled rows
- Options to restart or quit the game

### Gameplay
<img width="193" alt="Screenshot 2024-12-11 at 10 54 24 PM" src="https://github.com/user-attachments/assets/4b4726e9-b75e-463f-bc36-5501fcfcb71c" />


<img width="192" alt="Screenshot 2024-12-11 at 10 55 15 PM" src="https://github.com/user-attachments/assets/22d47137-e24e-4d87-93ce-6b01cfadd2af" />


## Installation
To run this Tetris game, you need to have Rust installed. You can install Rust from [here](https://www.rust-lang.org/).

```bash
# Clone the repository
git clone https://github.com/Human-Glitch/rust-playground.git

# Navigate to the tetris directory
cd rust-playground/tetris

# Run the game
cargo run
```

## Usage
Once the game starts, use the following controls:

- Arrow Keys: Move tiles left, right, and down
- Up Arrow: Rotate tiles
- R: Restart the game
- Q: Quit the game

## License
This project is licensed under the Apache License 2.0. See the [LICENSE](../LICENSE) file for more details.
