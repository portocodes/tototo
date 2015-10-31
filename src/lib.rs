#![allow(dead_code)]

#[derive(Copy,Clone)]
enum Player {
    FirstPlayer,
    SecondPlayer
}

type CellState = Option<Player>;

enum ErrorStates {
    IllegalMove
}

struct Board {
    cells: [CellState; 9],
    current_player: Player
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [None; 9],
            current_player: Player::FirstPlayer
        }
    }
}

impl Board {
    fn is_empty(self) -> bool {
        self.cells.iter().fold(true, |acc, &cell| acc && cell.is_none() )
    }

    fn at(self, x: usize, y: usize) -> CellState {
        self.cells[coord_to_flat(x,y)]
    }

    fn play(self, x: usize, y: usize) -> Result<Board,ErrorStates> {
        let current_cell = self.cells[coord_to_flat(x,y)];

        if current_cell.is_some() { return Err(ErrorStates::IllegalMove); }

        let next_player = match self.current_player {
            Player::FirstPlayer => Player::SecondPlayer,
            Player::SecondPlayer => Player::FirstPlayer
        };
        let mut next_cells: [CellState; 9] = self.cells.clone();
        next_cells[coord_to_flat(x,y)] = Some(self.current_player);

        let board = Board {
            cells: next_cells,
            current_player: next_player
        };

        Ok(board)
    }
}

fn coord_to_flat(x: usize, y: usize) -> usize {
    y*3+x
}

#[test]
fn it_initializes_board() {
    let board = Board::default();

    assert!(board.is_empty());
}

#[test]
fn it_gets_board_position() {
    let board = Board::default();

    assert!(board.at(0,0).is_none());
}

#[test]
fn it_allows_legal_move() {
    let result = Board::default().play(0,0);

    assert!(result.is_ok());
}

#[test]
fn it_sets_the_correct_player() {
    let board = Board::default().play(0,0).ok().unwrap();

    match board.at(0,0) {
        Some(Player::FirstPlayer) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn it_disallows_illegal_move() {
    let board = Board::default().play(0,0).ok().unwrap();

    match board.play(0,0) {
        Err(ErrorStates::IllegalMove) => assert!(true),
        _ => assert!(false)
    };
}
