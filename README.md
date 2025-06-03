# Conway's Game of Life
This project intends to be an MVP for the known [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

# Requirements
- cargo installed
- rustc installed
- make installed

# How to run
This project contains a Makefile that allows the user to build and run the game. The commands provided are:
- `make run`: compiles the game and runs it.
- `make build`: compiles the game
- `make test`: executes the unit tests for the game logic.
- `make clean`: deletes the target directory.

Run any of them from the project's root.

# How to play
- The user runs the game and an empty screen that reads "Choose some squares and press ENTER to start" appears.
- The user chooses as many cells as desired from the initial empty screen.
- The user presses the `enter` key to start the game.
- The game evolves on its own with the following rules:
    - For alive cells (painted):
        - With less than **two** alive neighbours (either horizontally, vertically or diagonally) -> cell dies by the next round
        - With **two or three** alive neighbours -> cell survives by the next round (remains painted)
        - With **more than three** alive neighbours -> cell dies by the next round
    - For dead cells (blank):
        - With **exactly three** alive neighbours -> cell is born by the next round

# Limitations and other considerations
- The present implementation is an MVP which means it has the minimal needed characteristics to be able to run.
- This implementation is attached to:
    - A _fixed sized screen_ and _does not support screen resizing_.
    - _Fixed sized cells_.
- This implementation contains a basic graphic interface.
- This implementation _does not support game restart_. In order to play again the game should be closed and started over.
- Currently, the cell entity is represented by its absolute position in the screen. It could be a good idea to redesign in the future.

# Future lines of work
If this project was to be developed deeper in the future, possible lines of action could be:
- Redesigning cells vs. screen sizes in order to make it more flexible.
- Redesign cell structure in order to be thought of from a relative position rather than an absolute position. This would affect calculations.
- Handle potential screen resizing.
- Create a more friendly user interface.
- Implement the possibility to pause the game's evolution.
- Implement the possibility to restart the game.

# The project structure:
- `src/lib.rs` contains all the code related to the logic of the game.
- `src/main.rs` contains all the code related to the graphic interface and the execution of the game.
