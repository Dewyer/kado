use crate::chess::piece::piece_position::{PiecePosition, PieceContext};
use crate::chess::piece::move_helper::ChessPieceMovementHelper;

pub fn get_all_straight_directions() -> Vec<(i32, i32)>
{
	vec![(1,0),(-1,0),(0,1),(0,-1)]
}

pub fn get_threatened_by_rook(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(true, get_all_straight_directions());
	helper.generate_positions(ctx.game,ctx.piece.position,true)
}

pub fn get_possible_moves_by_rook(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(true, get_all_straight_directions());
	helper.generate_positions(ctx.game,ctx.piece.position,false)
}