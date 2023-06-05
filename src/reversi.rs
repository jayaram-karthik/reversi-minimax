use std::fmt::Display;

const X1: [i8; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const Y1: [i8; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

const V_TABLE: [[f32; 8]; 8] = [
    [20., -3., 11., 8., 8., 11., -3., 20.],
    [-3., -7., -4., 1., 1., -4., -7., -3.],
    [11., -4., 2., 2., 2., 2., -4., 11.],
    [8., 1., 2., -3., -3., 2., 1., 8.],
    [8., 1., 2., -3., -3., 2., 1., 8.],
    [11., -4., 2., 2., 2., 2., -4., 11.],
    [-3., -7., -4., 1., 1., -4., -7., -3.],
    [20., -3., 11., 8., 8., 11., -3., 20.]
];

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Black,
    White,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, " "),
            CellState::Black => write!(f, "B"),
            CellState::White => write!(f, "W"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ReversiBoard {
    pub board: [[CellState; 8]; 8],
    pub current_player: CellState,
    pub last_move: Option<(usize, usize)>,
    turn: u8,
}

impl ReversiBoard {
    pub fn new() -> ReversiBoard {
        let mut board = [[CellState::Empty; 8]; 8];
        board[3][3] = CellState::White;
        board[3][4] = CellState::Black;
        board[4][3] = CellState::Black;
        board[4][4] = CellState::White;
        ReversiBoard {
            board: board,
            turn: 0,
            last_move: None,
            current_player: CellState::Black,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellState {
        self.board[y][x]
    }

    pub fn make_move(&mut self, x: usize, y: usize) -> bool {
        let color = match self.turn % 2 {
            0 => CellState::Black,
            1 => CellState::White,
            _ => panic!("this should never happen!"),
        };
        if self.check_if_move_valid(x, y, color) {
            // make move in othello
            self.board[y][x] = color;

            let opp_color = match color {
                CellState::Black => CellState::White,
                CellState::White => CellState::Black,
                CellState::Empty => panic!("this should never happen!"),
            };

            let tmp_x = x as i8;
            let tmp_y = y as i8;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if tmp_x + dx < 0 || tmp_x + dx > 7 || tmp_y + dy < 0 || tmp_y + dy > 7 {
                        continue;
                    }

                    if self.board[(tmp_y + dy) as usize][(tmp_x + dx) as usize] != opp_color {
                        continue;
                    }

                    let mut i = 2;

                    while i <= 7 {
                        if tmp_x + i * dx < 0 || tmp_x + i * dx > 7 || tmp_y + i * dy < 0 || tmp_y + i * dy > 7 {
                            break;
                        }

                        if self.board[(tmp_y + i * dy) as usize][(tmp_x + i * dx) as usize] == CellState::Empty{
                            break;
                        }

                        if self.board[(tmp_y + i * dy) as usize][(tmp_x + i * dx) as usize] == color {
                            let mut j = 1;
                            while j < i {
                                self.board[(tmp_y + j * dy) as usize][(tmp_x + j * dx) as usize] =
                                    color;
                                j += 1;
                            }
                            break;
                        }

                        i += 1;
                    }
                }
            }

            self.turn += 1;
            self.current_player = match self.current_player {
                CellState::Black => CellState::White,
                CellState::White => CellState::Black,
                CellState::Empty => panic!("this should never happen!"),
            };
            self.last_move = Some((x, y));
            true
        } else {
            false
        }
    }

    pub fn check_if_move_valid(&self, x: usize, y: usize, color: CellState) -> bool {
        if x > 7 || y > 7 {
            return false
        }
        
        if self.board[y][x] != CellState::Empty {
            return false;
        }

        let opp_color = match color {
            CellState::Black => CellState::White,
            CellState::White => CellState::Black,
            CellState::Empty => panic!("this should never happen!")
        };

        let tmp_x = x as i8;
        let tmp_y = y as i8;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if tmp_x + dx < 0 || tmp_x + dx > 7 || tmp_y + dy < 0 || tmp_y + dy > 7 {
                    continue;
                }

                if self.board[(tmp_y + dy) as usize][(tmp_x + dx) as usize] != opp_color {
                    continue
                }

                let mut i = 2;

                while (i <= 7) {
                    if (tmp_x + i * dx < 0 || tmp_x + i * dx > 7 || tmp_y + i * dy < 0 || tmp_y + i * dy > 7) {
                        break;
                    }

                    if self.board[(tmp_y + i * dy) as usize][(tmp_x + i * dx) as usize] == CellState::Empty {
                        break;
                    }

                    if self.board[(tmp_y + i * dy) as usize][(tmp_x + i * dx) as usize] == color {
                        return true;
                    }

                    i += 1;
                }
            }
        }

        false
    }

    pub fn get_possible_moves(&self, color: CellState) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                if self.check_if_move_valid(x, y, color) {
                    moves.push((x, y));
                }
            }
        }
        moves
    }


    fn get_mobility_score(&self, color: CellState) -> f32 {
        let opp_color = match color {
            CellState::Black => CellState::White,
            CellState::White => CellState::Black,
            _ => panic!("this should never happen!"),
        };

        let mut mobility = 0.0;

        let my_valid_moves_num = self.get_possible_moves(color).len();
        let opp_valid_moves_num = self.get_possible_moves(opp_color).len();

        if my_valid_moves_num > opp_valid_moves_num {
            mobility = 100f32 * my_valid_moves_num as f32 / (my_valid_moves_num + opp_valid_moves_num) as f32;
        } else if opp_valid_moves_num > my_valid_moves_num {
            mobility = -100f32 * opp_valid_moves_num as f32 / (my_valid_moves_num + opp_valid_moves_num) as f32;
        }

        mobility
    }

    fn get_piece_diff_frontier_scores(&self, color: CellState) -> (f32, f32, f32) {
        let opp_color = match color {
            CellState::Black => CellState::White,
            CellState::White => CellState::Black,
            _ => panic!("this should never happen!"),
        };

        let mut my_pieces = 0;
        let mut opp_pieces = 0;
        let mut my_front_pieces = 0;
        let mut opp_front_pieces = 0;

        let mut diff_score = 0.0;
        let mut piece_score = 0.0;
        let mut frontier_score = 0.0;

        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j] == color {
                    diff_score += V_TABLE[i][j];
                    my_pieces += 1;
                } else if self.board[i][j] == opp_color {
                    diff_score -= V_TABLE[i][j];
                    opp_pieces += 1;
                }

                if self.board[i][j] != CellState::Empty {
                    for k in 0..8 {
                        let x = i as i8 + X1[k];
                        let y = j as i8 + Y1[k];

                        if x >= 0 && x < 8 && y >= 0 && y < 8 {
                            if self.board[x as usize][y as usize] != CellState::Empty {
                                if self.board[i][j] == color {
                                    my_front_pieces += 1;
                                } else {
                                    opp_front_pieces += 1;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        if my_pieces > opp_pieces {
            piece_score = 100f32 * my_pieces as f32 / (my_pieces + opp_pieces) as f32;
        } else if opp_pieces > my_pieces {
            piece_score = -100f32 * opp_pieces as f32 / (my_pieces + opp_pieces) as f32;
        }

        if my_front_pieces > opp_front_pieces {
            frontier_score = -100f32 * my_front_pieces as f32 / (my_front_pieces + opp_front_pieces) as f32;
        } else if opp_front_pieces > my_front_pieces {
            frontier_score = 100f32 * opp_front_pieces as f32 / (my_front_pieces + opp_front_pieces) as f32;
        }

        (diff_score, piece_score, frontier_score)
    }

    fn get_corner_occupancy_score(&self, color: CellState) -> f32 {
        let opp_color = match color {
            CellState::Black => CellState::White,
            CellState::White => CellState::Black,
            _ => panic!("this should never happen!"),
        };

        let mut my_corners = 0;
        let mut opp_corners = 0;

        if self.board[0][0] == color {
            my_corners += 1;
        } else if self.board[0][0] == opp_color {
            opp_corners += 1;
        }

        if self.board[0][7] == color {
            my_corners += 1;
        } else if self.board[0][7] == opp_color {
            opp_corners += 1;
        }

        if self.board[7][0] == color {
            my_corners += 1;
        } else if self.board[7][0] == opp_color {
            opp_corners += 1;
        }

        if self.board[7][7] == color {
            my_corners += 1;
        } else if self.board[7][7] == opp_color {
            opp_corners += 1;
        }

        (25 * (my_corners - opp_corners)) as f32
    }

    fn get_corner_closeness_score(&self, color: CellState) -> f32 {
        let opp_color = match color {
            CellState::Black => CellState::White,
            CellState::White => CellState::Black,
            _ => panic!("this should never happen!"),
        };

        let mut my_corners = 0;
        let mut opp_corners = 0;

        if self.board[0][0] == CellState::Empty {
            if self.board[0][1] == color {
                my_corners += 1;
            } else if self.board[0][1] == opp_color {
                opp_corners += 1;
            }

            if self.board[1][1] == color {
                my_corners += 1;
            } else if self.board[1][1] == opp_color {
                opp_corners += 1;
            }

            if self.board[1][0] == color {
                my_corners += 1;
            } else if self.board[1][0] == opp_color {
                opp_corners += 1;
            }
        }

        if self.board[0][7] == CellState::Empty {
            if self.board[0][6] == color {
                my_corners += 1;
            } else if self.board[0][6] == opp_color {
                opp_corners += 1;
            }

            if self.board[1][6] == color {
                my_corners += 1;
            } else if self.board[1][6] == opp_color {
                opp_corners += 1;
            }

            if self.board[1][7] == color {
                my_corners += 1;
            } else if self.board[1][7] == opp_color {
                opp_corners += 1;
            }
        }

        if self.board[7][0] == CellState::Empty {
            if self.board[6][0] == color {
                my_corners += 1;
            } else if self.board[6][0] == opp_color {
                opp_corners += 1;
            }

            if self.board[6][1] == color {
                my_corners += 1;
            } else if self.board[6][1] == opp_color {
                opp_corners += 1;
            }

            if self.board[7][1] == color {
                my_corners += 1;
            } else if self.board[7][1] == opp_color {
                opp_corners += 1;
            }
        }

        if self.board[7][7] == CellState::Empty {
            if self.board[6][7] == color {
                my_corners += 1;
            } else if self.board[6][7] == opp_color {
                opp_corners += 1;
            }

            if self.board[6][6] == color {
                my_corners += 1;
            } else if self.board[6][6] == opp_color {
                opp_corners += 1;
            }

            if self.board[7][6] == color {
                my_corners += 1;
            } else if self.board[7][6] == opp_color {
                opp_corners += 1;
            }
        }

        -12.5 * (my_corners - opp_corners) as f32
    }

    pub fn get_piece_count(&self, color: CellState) -> usize {
        let mut count = 0;
        for row in self.board.iter() {
            for cell in row.iter() {
                if *cell == color {
                    count += 1;
                }
            }
        }
        count
    }

    // SOURCE: https://courses.cs.washington.edu/courses/cse573/04au/Project/mini1/RUSSIA/Final_Paper.pdf
    pub fn get_board_eval(&self, color: CellState) -> f32 {
        if self.get_possible_moves(color).len() == 0 {
            let opp_color = match color {
                CellState::Black => CellState::White,
                CellState::White => CellState::Black,
                _ => panic!("this should never happen!"),
            };

            if self.get_piece_count(color) > self.get_piece_count(opp_color) {
                return f32::MAX;
            } else if self.get_piece_count(color) < self.get_piece_count(opp_color) {
                return f32::MIN;
            } else {
                return 0f32;
            }

        }

        let (diff_score, piece_score, frontier_score) = self.get_piece_diff_frontier_scores(color);
        let corner_occupancy_score = self.get_corner_occupancy_score(color);
        let corner_closeness_score = self.get_corner_closeness_score(color);
        let mobility_score = self.get_mobility_score(color);

        10f32 * piece_score + 801.724 * corner_occupancy_score + 382.026 * corner_closeness_score + 78.922 * mobility_score + 74.396 * frontier_score + 10f32 * diff_score
    }

    pub fn is_board_terminal(&self, color: CellState) -> bool {
        self.get_possible_moves(color).len() == 0
    }

    pub fn get_children_nodes(&self) -> Vec<ReversiBoard> {
        let mut children = Vec::new();
        for (i, j) in self.get_possible_moves(self.current_player) {
            let mut new_board = self.clone();
            new_board.make_move(i, j);
            children.push(new_board);
        }
        children
    }

    pub fn get_winner(&self) -> CellState {
        let black_count = self.get_piece_count(CellState::Black);
        let white_count = self.get_piece_count(CellState::White);

        if black_count > white_count {
            CellState::Black
        } else if white_count > black_count {
            CellState::White
        } else {
            CellState::Empty
        }
    }
}

impl Display for ReversiBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&"  +----------------+\n");
        for (i, row) in self.board.iter().enumerate() {
            s.push_str(&format!("{} |", i+1));
            for cell in row {
                s.push_str(&format!("{} ", cell));
            }
            s.push_str(&"|\n")
        }
        s.push_str(&"  +----------------+\n");

        write!(f, "{}", s)
    }
} 
