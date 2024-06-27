<div align="center">

<img
  width="200"
  alt="Javelin Logo"
  src=".readme/logo/logo_small_rc.png">
 
<h3>Javelin</h3>
<b>Apparently, "MCTS is cool.", so let's see :)</b>
<br>
<br>

[![License](https://img.shields.io/github/license/TomaszJaworski777/Javelin?style=for-the-badge)](https://opensource.org/license/mit)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/TomaszJaworski777/Javelin?style=for-the-badge)](https://github.com/TomaszJaworski777/Javelin/releases/latest)
[![Commits](https://img.shields.io/github/commits-since/TomaszJaworski777/Javelin/latest?style=for-the-badge)](https://github.com/TomaszJaworski777/Javelin/commits/main)
<br>
<br>

| Version | CCRL 40/15 | CCRL Blitz | Estimated | Release Date |
| :-: | :-: | :-: | :-: | :-: |
| [1.0.0](https://github.com/TomaszJaworski777/Javelin/releases/tag/1.0.0) | - | 1830 | 1798 | 31th May 2024 |
| [2.0.0](https://github.com/TomaszJaworski777/Javelin/releases/tag/2.0.0) | - | - | 2539 | 27th June 2024 |

</div>

## Overview
Javelin is a second UCI chess engine I made. It uses Monte Carlo Tree Search (MCTS) to find best moves. I made it as an experiment, wanting to try something different with MCTS. Javelin works with any chess GUI that supports UCI. Data for training value and policy neural networks was generated through entirely through selfplay from the beginning, when Javelin was using basic PeSTO. 

## Compiling
To compile Javelin, follow these steps after downloading the source code from the release:

1. Open folder `Javelin-X.X.X`
2. Inside this folder create directory `target` and inside this directory create another one called `builds`
3. Go back to root directory and start a terminal
4. Start a terminal in root folder and enter the `make` command

## Credits
Javelin is developed by Tomasz Jaworski. Special thanks to:

* [@jw1912](https://github.com/jw1912) for mentoring me through the process
* [@jw1912](https://github.com/jw1912) for creating [Monty](https://github.com/jw1912/monty/tree/main) chess engine that provided immense help with understanding optimized algorithms
* [@AndyGrant](https://github.com/AndyGrant) for letting me borrow his SEE implementation
* [@princesslana](https://github.com/princesslana) for helping with subnet policy trainer
* [@jw1912](https://github.com/jw1912) for creating [goober](https://github.com/jw1912/goober), that I used for policy training and inference

## Command List
Javelin supports all necessary commands to initialize UCI protocol, full description of the protocol can be found [here](https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf).
* `go <wtime> <btime> <winc> <binc> <movestogo> <depth> <nodes> <movetime> <infinite>` - Starts the search with provided parameters.
* `position <fen|startpos> <FEN> moves <moves>` - Creates new board and sets it for the engine.
* `stop` - Stops the search.
* `quit` - Exists the engine.
* `draw` - Draws the board in the terminal.
* `tree <depth>` - Draws tree of most recent search.
* `tree <depth> <node>` - Draws tree of most recent search from provided node index.
* `perft <depth>` - Runs perft test on current position.
* `bulk <depth>` - Runs perft test on current position in bulk mode.
* `bench <depth>` - Runs benchmark to test engine speed.

## Feature List
* MCTS Search
   * Tree Reuse
   * Flatten policy at root
   * Replacement of least recently used node
   * First play urgency
   * Scaling C with search duration
* Quiescence Search
   * MVV-LVA
   * Static Exchange Evaluation
* Value Network
   * Architecture: 768->128->1
   * Horizontal mirroring based on kings file
* Policy Network
   * Architecture: 128 subnets pairs (768->16)
   * Selecting subnet pair for move destination based on SEE result
