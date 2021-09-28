use crate::chess::game::{ChessGame, GameState};
use crate::chess::piece::{PieceType, ChessPiece};
use crate::chess::piece::piece_position::{PiecePosition, PlayerMove};

pub const CHESS_PIECES_BASE_UNICODE_VALUE:u32 = 9812;

pub struct ConsoleChessGame
{
    chess_game:ChessGame
}

impl ConsoleChessGame
{
    pub fn new() -> Self
    {
        Self
        {
            chess_game:ChessGame::new()
        }
    }

    pub fn get_unicode_representation_of_piece(piece:&ChessPiece) -> String
    {
        if piece.piece_type == PieceType::Empty
        {
            return "▯".to_string();
        }
        let piece_order = vec![PieceType::King, PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight, PieceType::Pawn];
        let index: u32 = piece_order.iter().position(|el| std::mem::discriminant(el) == std::mem::discriminant(&piece.piece_type)).unwrap() as u32;
        let code_point = CHESS_PIECES_BASE_UNICODE_VALUE + index + (if piece.owner.is_white() { 0 } else { 6 });
        std::char::from_u32(code_point).unwrap().to_string()
    }

    pub fn display_on_console(&self)
    {
        for row in 0..8
        {
            print!("|");
            for col in 0..8
            {
                let at_piece =  &self.chess_game.board[row][col];
                print!("{}|",Self::get_unicode_representation_of_piece(at_piece));
            }
            println!();
        }
    }

    fn clear_screen()
    {
        for _ in 0..80
        {
            println!();
        }
    }

    fn read_piece_position() -> Option<PiecePosition>
    {
        loop {
            let res =Self::read_line();
            if res == "q"
            {
                return None;
            }
            if let Ok(vv) = PiecePosition::new_from_notation(res)
            {
                return Some(vv);
            }
            else
            {
                println!("Thats not a valid position, try again: ");
            }
        }
    }

    fn read_line() -> String
    {
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp).expect("Cant read line!");
        let res = inp.trim_end();
        res.to_string()
    }

    fn read_int(max_v:i32) -> Option<i32>
    {
        loop
        {
            let line = Self::read_line();
            if line == "q"
            {
                return None;
            }
            if let Ok(index) = line.parse::<i32>()
            {
                if index >= 0 && index <= max_v
                {
                    return Some(index);
                }
            }
            else
            {
                println!("Please enter a index.");
            }
        }
    }

    pub fn try_make_move_for_piece(&mut self) -> Option<()>
    {
        Self::clear_screen();
        self.display_on_console();
        println!("{:?} - select a piece to move :",self.chess_game.on_turn);
        let selected_pos = Self::read_piece_position()?;
        let selected_piece = self.chess_game.get_piece_at(selected_pos);
        let pos_m = self.chess_game.get_moves_for_piece(selected_piece);
        println!("{:?} piece selected",selected_piece);
        println!("=====:");
        for (ii,pp) in pos_m.iter().enumerate()
        {
            println!("{} : {}",pp.get_notation(),ii);
        }
        println!("Select a move :");
        let move_index = Self::read_int(pos_m.len() as i32-1)?;

        self.chess_game.make_move(PlayerMove{
            owner:self.chess_game.on_turn.clone(),
            from:selected_pos,
            to:pos_m[move_index as usize]
        }).unwrap();

        Some(())
    }

    pub fn play_loop(&mut self)
    {
        loop
        {
            if self.chess_game.state != GameState::Playing
            {
                break;
            }
            self.try_make_move_for_piece();
        }

        println!("{:?} was the final state!",self.chess_game.state);
    }

}