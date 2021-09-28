use crate::chess::game::{ChessBoard, Player};
use crate::chess::piece::{ChessPiece, PieceType};
use crate::chess::piece::piece_position::PiecePosition;

pub fn construct_base_row(is_white: bool) -> Vec<ChessPiece>
{
	let piece_order = vec![PieceType::Rook, PieceType::Bishop, PieceType::Knight, PieceType::King, PieceType::Queen, PieceType::Knight, PieceType::Bishop, PieceType::Rook];
	let mut row: Vec<ChessPiece> = vec![];
	for ii in 0..8
	{
		let target_type = piece_order[if is_white { 7 - ii } else { ii }].clone();
		let pos = PiecePosition::new_from_cord(if is_white { 7 } else { 0 },ii).unwrap();
		row.push(ChessPiece::new(Player::from_bool(is_white), target_type, pos));
	}

	row
}

pub fn construct_starter_board() -> ChessBoard
{
	let mut board: ChessBoard = vec![];
	for row in 0..8
	{
		if row == 0 || row == 7
		{
			board.push(construct_base_row(row == 7));
		} else if row == 1 || row == 6
		{
			//ChessPiece{is_white:row == 6,piece_type:PieceType::Pawn,game:Rc::new(0)}
			board.push((0..8).map(|col| {
				let pos = PiecePosition::new_from_cord(row, col).unwrap();
				ChessPiece::new(Player::from_bool(row == 6), PieceType::Pawn, pos)
			}).collect());
		} else {
			board.push((0..8).map(|col| ChessPiece::empty(PiecePosition::new_from_cord(row, col).unwrap())).collect());
		}
	}

	board
}