In Conway's Game of Life, a 2-D board is populated with live or dead cells. Each board square can only have one cell. Specific circumstances determine whether a cell will
remain alive, become alive, die, or remain dead. The rules are as follows:
If a cell has 2 neighbors -> The target stays as is (Alive -> Alive, Dead -> Dead)
If a cell has 3 neighbors -> The target becomes alive (Alive -> Alive, Dead -> Alive)
Any other amount of neighbors -> The target becomes dead (Alive -> Dead, Dead -> Dead)
Note: Diagonal cells are also neighbors
In each iteration of the game, all board squares are evaluated. The included code computes an alogrithm for the game and prints out a sample 10 generations with arbitrary starting cells.
