use crate::chess::piece::piece_position::{PiecePosition, PieceContext};
use crate::chess::piece::move_helper::ChessPieceMovementHelper;

use crate::chess::piece::queen::get_all_directions;

pub fn get_threatened_by_king(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(false, get_all_directions());
	helper.generate_positions(ctx.game,ctx.piece.position,true)
}

pub fn get_possible_moves_by_king(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(false, get_all_directions());
	helper.generate_positions(ctx.game,ctx.piece.position,false)
}