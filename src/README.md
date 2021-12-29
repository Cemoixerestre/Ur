# Experimentations with Computer Strategies for the Royal Game of Ur

The goal of this folder is to experiment with reinforcement learning and
expectimax algorithm in order to improve computer playing for the game of Ur.

Structure of the repository:
- `game_manager.rs`: contains the data-structure representing the board.
- `strategy.rs`: definition of heuristic and expectimax algorithm.
- `tournament.rs`: contains the infrastructure to run a tournament between IAs.
- `reinforcement_learning.rs`: contains the definition of a trainable heuristic
   and the infrastracture to train it.
Exectutables:
- `main.rs`: runs tournaments between some pre-defined players.
- `training.rs`: runs a training session

TODO:
- [ ] write a test module
- [ ] create a benchmark
- [ ] make the computation of the heuristics more efficient
