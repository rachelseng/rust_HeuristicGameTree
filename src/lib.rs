use core::isize::{MAX,MIN};
use std::io::{stdin,stdout,Write};
use std::cmp::max;


const TICWINS: [[usize; 3]; 8] = [[0, 1, 2], [0, 3, 6], [0, 4, 8], [1, 4, 7], [2, 5, 8], [2, 4, 6], [3, 4, 5], [6, 7, 8]];
//const ROWS: [&str; 3] = ["A", "B", "C"];


pub fn lets_play(game: usize, diff: usize)
{
    match game
        {
            1 => starttic(diff),
            2 => println!("connect4"),
            3 => println!("checkers"),
            _ => println!("error"),
        }
}
#[derive (PartialEq, Clone)]
pub enum Piece {
    X,
    O,
    None,
    Tie
}

impl Piece {
    fn is_x(&self) -> bool {
        if let Piece::X = self {
            true
        }
        else {false}
    }
    fn is_o(&self) -> bool {
        if let Piece::O = self {
            true
        }
        else {false}
    }
    fn is_none(&self) -> bool {
        if let Piece::None = self {
            true
        }
        else {false}
    }
    fn val(&self) -> &str
    {
        if self.is_x()
        {
         return "X"
        }
        if self.is_o()
        {
            return "O"
        }
        if self.is_none()
        {
            return ""
        }
        "Tie"
    }
}

#[allow(unused)]
pub trait HeuristicGameTree: Clone {
    type Move: Clone;
    // Should be a type Heuristic that can be any type that can be compared; doing isize for now

    fn possible_moves(&mut self) -> Vec<Self::Move>;
    fn heuristic(&self) -> isize;
    fn execute_move(&mut self, next_move: Self::Move, is_opponent: bool);

    fn minimax_search(&mut self, depth: usize, is_opponent: bool) -> Option<Self::Move> {
        // Gets the possible moves (i.e. children)
        // Makes executes each move on a copy of the game
        // Gets the heuristic of each game copy executed on a next move
        // by calling minimax again
        // Returns the move corresponding with the best heuristic
        let mut best_move = (None,MIN); // We're going to maximize heuristic
        if depth > 0 {
            for mymove in self.possible_moves() {
                let mut next_state = self.clone();
                next_state.execute_move(mymove.clone(), !is_opponent); // Need to clone, standard procedure with minimax
                let h = next_state.minimax_helper(depth-1, false, MAX, MIN);
                if h > best_move.1 {
                    best_move = (Some(mymove),h);
                }
            }
        }
        else {
            // Choose first available move.
            let moves = self.possible_moves();
            if moves.len() == 0 {
                return None;
            }
            return Some(moves[0].clone());
        }

        best_move.0 // Return the move that corresponds with best heuristic
    }
    // Need a helper cause the client shouldn't provide alpha and beta
    // also nice because we don't have to have return valus of structs/tuples, can just do an isize
    // that corresponds to the best value for the immediately next move
    fn minimax_helper(&mut self, depth: usize, is_opponent: bool, mut alpha: isize, mut beta: isize) -> isize {
        let mut current_heuristic = self.heuristic();
        if depth > 0 { // End of depth, return
            if is_opponent { // Maximizing
                let mut child_heuristic = MIN;
                for mymove in self.possible_moves() {
                    let mut next_state = self.clone();
                    next_state.execute_move(mymove.clone(), !is_opponent);
                    let h = next_state.minimax_helper(depth-1, false, alpha, beta);
                    if h > child_heuristic {
                        child_heuristic = h;
                    }
                    if h > alpha {
                        alpha = h;
                    }
                    if beta < alpha {break;}
                }
                if child_heuristic > MIN {
                    current_heuristic = child_heuristic;
                }
            }
            else { // Minimizing
                let mut child_heuristic = MAX;
                for mymove in self.possible_moves() {
                    let mut next_state = self.clone();
                    next_state.execute_move(mymove.clone(), !is_opponent);
                    let h = next_state.minimax_helper(depth-1, false, alpha, beta);
                    if h < child_heuristic {
                        child_heuristic = h;
                    }
                    if h > beta {
                        beta = h;
                    }
                    if beta < alpha {break;}
                }
                if child_heuristic < MAX {
                    current_heuristic = child_heuristic;
                }
            }
        }
        current_heuristic
    }
}

impl<'a> HeuristicGameTree for TicGame<'a> {
    type Move = usize;
    fn possible_moves(&mut self) -> Vec<Self::Move> {
        let mut list = Vec::new();
        for i in 0..9 {
            if self.board[i] == &Piece::None {
                list.push(i);
            }
        }
        list
    }
    // MAKE THIS BETTER
    fn heuristic(&self) -> isize {
        // invariant: x_streak != o_streak != 3
        // keep track of best streak by each player
        // 
        let mut x_streak = if self.moves > 0 {1} else {0};
        let mut o_streak = if self.moves > 1 {1} else {0};
        // First check for wins
        if self.board[4] != &Piece::None {
            let center = self.board[4];
            if center == self.board[0] && center == self.board[8] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[1] && center == self.board[7] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[2] && center == self.board[6] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[3] && center == self.board[5] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        if self.board[0] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[1] && corner == self.board[2] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if corner == self.board[3] && corner == self.board[6] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        if self.board[8] != &Piece::None {
            let corner = self.board[8];
            if corner == self.board[5] && corner == self.board[2] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if corner == self.board[7] && corner == self.board[6] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        // Do pairs
        if self.board[4] != &Piece::None {
            let center = self.board[4];
            for i in 0..9 {
                if i != 4 && center == self.board[i] { // found a pair
                    if center == &Piece::X {
                        x_streak = max(x_streak,2);
                    }
                    else {
                        o_streak = max(o_streak,2);
                    }
                }
            }
        }
        if self.board[0] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[1] || corner == self.board[3] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[2] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[1] || corner == self.board[5] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[6] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[7] || corner == self.board[3] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[8] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[7] || corner == self.board[5] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        x_streak - o_streak // Why is this backwards?
    }
    fn execute_move(&mut self, next_move: Self::Move, is_opponent: bool) {
        self.store_move(next_move, if is_opponent {&Piece::O} else {&Piece::X});
    }
}

#[derive (Clone)]
struct TicGame<'a>
{
    board: Vec<&'a Piece>,
    winner: Option<Piece>,
    moves: usize,
}

impl<'a> TicGame<'a>
{
    fn new() -> Self
    {

        TicGame {
            board: vec![&Piece::None; 9],
            winner: None,
            moves: 0,
        }
    }

    fn printboard(& mut self)
    {
        println!("  1  2  3");
        println!("A {}  {}  {}", self.board[0].val(), self.board[1].val(), self.board[2].val());
        println!("B {}  {}  {}", self.board[3].val(), self.board[4].val(), self.board[5].val());
        println!("C {}  {}  {}", self.board[6].val(), self.board[7].val(), self.board[8].val());
    }

    fn validmove(self, row: &'a str, col: usize) -> (bool, usize)
    {
        if (row == "A" || row == "B" || row == "C") && (col == 1 || col == 2 || col == 3) {
            let int = match row {
                "A" => 0,
                "B" => 3,
                "C" => 6,
                _ => 100,
            };

            let num = int + col - 1;

            if self.board[num] == &Piece::None {
                return (true, num)
            }
        }

        (false, 10)
    }

    fn store_move(&mut self, position:usize, player: &'a Piece){
        self.board[position] = player;
        self.moves += 1;
        if self.moves == 9
        {
            self.winner = Some(Piece::Tie);
        }
    }

    fn check_win(& mut self, player: Piece) -> bool
    {
        for vecs in TICWINS.iter()
            {
                let mut in_row = 0;
                for index in vecs.iter()
                    {
                        if self.board[*index] == &player
                        {
                            in_row += 1;
                        }
                    }
                if in_row == 3
                {
                  return true
                }
            }
        false
    }

}

fn starttic(difficulty: usize)
{
    let mut new_game = TicGame::new();

    while new_game.winner == None {
        println!("Where do you want to put your X? (row then column)");
        new_game.printboard();
        let mut loc = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut loc).expect("Did not enter a correct string");
        let mut loc = loc.split_whitespace();
        if loc.clone().count() == 2 {
            let row =  loc.next().unwrap();
            let col =  loc.next().unwrap();
                let (valid, pos) = new_game.clone().validmove(row, col.parse().unwrap()); //throw error
                if valid
                {
                    // pos is our move, store_move is our execute
                    new_game.store_move(pos, &Piece::X);
                    if new_game.check_win(Piece::X)
                    {
                        new_game.winner = Some(Piece::X);
                    } else {
                        let next_move = new_game.clone().minimax_search(&difficulty * 3, true);
                        if let Some(m) = next_move {
                            new_game.store_move(m, &Piece::O);
                            if new_game.check_win(Piece::O) {
                                new_game.winner = Some(Piece::O);
                            }
                        }
                        // let mut cont = true;
                        // for row in ROWS.iter() {
                        //     for col in 1..3
                        //         {
                        //             let (valid2, pos2) = new_game.clone().validmove(*row, col);
                        //             if valid2 & cont
                        //             {
                        //                 new_game.store_move(pos2, &Piece::O);
                        //                 if new_game.check_win(Piece::O)
                        //                 {
                        //                     new_game.winner = Some(Piece::O);
                        //                 }
                        //                 cont = false;
                        //             }
                        //         }
                        // }
                    }
                }
                else {
                    println!("That is not a valid move! Try again");
                }
            }
        println!("You did not input your move correctly! Try again");
        }
    new_game.printboard();
    println!("{} WON THE GAME!", new_game.winner.unwrap().val());
    }

