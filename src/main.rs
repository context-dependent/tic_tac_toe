use std::fmt;
use std::io;

const SIZE: usize = 3;

#[derive(PartialEq, Clone, Copy)]
enum Square {
    X,
    O,
    Empty,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Square::X => "X",
            Square::O => "O",
            Square::Empty => " ",
        };
        write!(f, "{}", symbol)
    }
}

impl Square {
    fn flip(&self) -> Square {
        match self {
            Square::X => Square::O,
            Square::O => Square::X,
            Square::Empty => Square::Empty,
        }
    }
}

type Board = [[Square; SIZE]; SIZE];

struct Game {
    board: Board,
    player: Square,
    human: Square,
    agent: Square,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[Square::Empty; SIZE]; SIZE],
            player: Square::X,
            human: Square::X,
            agent: Square::O,
        }
    }

    fn draw_board(&self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("|{}", self.board[i][j]);
            }
            println!("|");
        }
    }

    fn is_winner(&self, player: Square) -> bool {
        // Check if the board is won by the player that just played
        //  - Check if any row is all the same as the player that just played
        //  - It's impossible to win before three moves have been made by the same player,
        //    so we don't need to check until the fifth total turn
        //  - We can use the `all` method on iterators to check if all the squares in a row
        //    have been filled by the player that just played
        let win_horizontal = self
            .board
            .iter()
            .any(|row| row.iter().all(|&square| square == player));
        let win_vertical = (0..SIZE).any(|col| self.board.iter().all(|row| row[col] == player));
        let win_diagonal_down = (0..SIZE).all(|i| self.board[i][i] == player);
        let win_diagonal_up = (0..SIZE).all(|i| self.board[i][SIZE - i - 1] == player);
        win_horizontal || win_vertical || win_diagonal_down || win_diagonal_up
    }

    fn is_draw(&self) -> bool {
        // Check if the board is full
        //  - We can use the `all` method on iterators to check if all the squares in the board
        //    have been filled
        self.board
            .iter()
            .all(|row| row.iter().all(|&square| square != Square::Empty))
    }

    fn make_move(&mut self, row: usize, col: usize) -> bool {
        match self.board[row][col] {
            Square::Empty => {
                self.board[row][col] = self.player;
                self.player = self.player.flip();
                true
            }
            _ => false,
        }
    }
}

fn minimax(game: &mut Game, depth: usize, is_maximizing: bool) -> (i32, (usize, usize)) {
    let mut best_move: (usize, usize) = (1, 1);
    let mut score: i32;

    if game.is_winner(game.agent) {
        return (1, best_move);
    } else if game.is_winner(game.human) {
        return (-1, best_move);
    } else if game.is_draw() {
        return (0, best_move);
    }

    if is_maximizing {
        let mut best_score = -1000;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if game.board[i][j] == Square::Empty {
                    game.board[i][j] = game.agent;
                    (score, _) = minimax(game, depth + 1, false);
                    game.board[i][j] = Square::Empty;
                    if score > best_score {
                        best_score = score;
                        best_move = (i, j);
                    }
                }
            }
        }
        return (best_score, best_move);
    } else {
        let mut best_score = 1000;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if game.board[i][j] == Square::Empty {
                    game.board[i][j] = game.human;
                    (score, _) = minimax(game, depth + 1, true);
                    game.board[i][j] = Square::Empty;
                    best_score = best_score.min(score);
                }
            }
        }
        return (best_score, best_move);
    }
}

fn main() {
    let mut game = Game::new();

    println!("Welcome to Tic Tac Toe! The board is numbered like this:");
    println!("  0 1 2");
    println!("0| | | |");
    println!("1| | | |");
    println!("2| | | |");
    println!("You will enter your moves in the form `row col`.");

    println!("Please choose your symbol: X or O.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    if input.trim() == "O" {
        game.human = Square::O;
        game.agent = Square::X;
    }

    loop {
        let valid_move;

        if game.player == game.agent {
            let (_, (row, col)) = minimax(&mut game, 0, true);
            valid_move = game.make_move(row, col)
        } else {
            game.draw_board();
            println!(
                "Player {}, please enter your move in the form `row col`.",
                game.player
            );
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let mut coords = input.split_whitespace();
            let row = coords.next().unwrap().parse().unwrap();
            let col = coords.next().unwrap().parse().unwrap();
            valid_move = game.make_move(row, col);
        }

        if valid_move {
            if game.is_winner(game.player) {
                println!("Player {} wins!", game.player);
                break;
            } else if game.is_draw() {
                println!("The game is a draw!");
                break;
            }
        } else {
            println!("Invalid move, please try again.");
        }
    }
}
