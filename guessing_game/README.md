# Guessing Game

This is a simple guessing game written in Rust. The program generates a random number between 1 and 100, and the player has to guess the number. After each guess, the program provides feedback on whether the guess was too high, too low, or correct.

## Installation

To run this game, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and navigate to the guessing_game directory:

```sh
git clone https://github.com/Human-Glitch/rust-playground.git
cd rust-playground/guessing_game
```

## Usage
To build and run the guessing game, use the following commands:

```sh
cargo build
cargo run
```
The game will prompt you to guess a number between 1 and 100. Enter your guess and the program will tell you if your guess is too high, too low, or correct. The game continues until you guess the correct number.

## Example
```
Guess the number!
Please input your guess
50
You guessed: 50
Too small!
Please input your guess
75
You guessed: 75
Too big!
Please input your guess
63
You guessed: 63
You win!
```

## License
This project is licensed under the Apache License 2.0. See the [LICENSE](../LICENSE) file for more details.
