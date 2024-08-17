use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

type Board = [char; BOARD_SIZE * BOARD_SIZE];

fn initialize_board() -> Board {
    let mut board = [' '; BOARD_SIZE * BOARD_SIZE];
    for i in 0..BOARD_SIZE * BOARD_SIZE {
        board[i] = (i as u8 + b'1') as char;
    }
    board
}

fn print_board(board: &Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            print!(" {} ", board[row * BOARD_SIZE + col]);
            if col < BOARD_SIZE - 1 {
                print!("|");
            }
        }
        println!();
        if row < BOARD_SIZE - 1 {
            println!("---+---+---");
        }
    }
}

fn get_player_move(current_player: char, board: &Board) -> usize {
    loop {
        println!("Player {} input (1-9):", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(index) = input.trim().parse::<usize>() {
            if index >= 1 && index <= 9 && board[index - 1] != PLAYER_X && board[index - 1] != PLAYER_O {
                return index - 1;
            }
        }
        println!("Invalid input, please enter a valid number between 1 and 9.");
    }
}

fn check_winner(current_player: char, board: &Board) -> bool {
    let win_patterns = [
        [0, 1, 2], // Row 1
        [3, 4, 5], // Row 2
        [6, 7, 8], // Row 3
        [0, 3, 6], // Column 1
        [1, 4, 7], // Column 2
        [2, 5, 8], // Column 3
        [0, 4, 8], // Diagonal 1
        [2, 4, 6], // Diagonal 2
    ];

    for pattern in win_patterns.iter() {
        if board[pattern[0]] == current_player
            && board[pattern[1]] == current_player
            && board[pattern[2]] == current_player
        {
            return true;
        }
    }
    false
}

fn check_draw(board: &Board) -> bool {
    board.iter().all(|&cell| cell == PLAYER_X || cell == PLAYER_O)
}

fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current Board:");
        print_board(&board);

        let index = get_player_move(current_player, &board);
        board[index] = current_player;

        if check_winner(current_player, &board) {
            println!("Player {} is the winner!", current_player);
            print_board(&board);
            break;
        }

        if check_draw(&board) {
            println!("The game is a draw.");
            print_board(&board);
            break;
        }

        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}

fn main() {
    println!("Welcome to the Tic Tac Toe Game");
    play_game();
}
