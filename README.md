# Terminal-Sudoku
# A simple Sudoku game running in a terminal for the Hackclubs' Summer Of Making 2025
> LOC: 662
## Features:
- Playing Sudoku in the Terminal
- Detecting when you've messed it up (you only have 1 try!)
- Settings (via flags):
  - stopwatch: Keep track of how much time you needed, saves the best one!
  - sudoku_maker: Save your game in a string
  - templates: use a saved game ("state code") to apply it to the current game. It's possible to use your own pre-made template (but not really recommended)
- Stats, so you can keep track of your victories (and failures) each session
- Nice user interface (always a "clean screen" after each step)
- Written in Rust, so it's pretty performant and lightweight (~1.25mb with the --release flag) and, if anyone wants that (I would wonder), it's pretty memory safe too!

> ## How to install
> - [SoM link](https://summer.hackclub.com/projects/1074)
> - [Rust Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e2c0c20617d25444381b442eb1386cfb)
> - `cargo install TerminalSudoku` if you have Rust installed
