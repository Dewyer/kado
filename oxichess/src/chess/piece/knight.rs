use crate::chess::piece::piece_position::{PiecePosition, PieceContext};
use crate::chess::piece::move_helper::ChessPieceMovementHelper;

pub fn get_all_knight_vectors() -> Vec<(i32, i32)>
{
	vec![(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(-1,2),(1,-2),(-1,-2)]
}

pub fn get_threatened_by_knight(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(false, get_all_knight_vectors());
	helper.generate_positions(ctx.game,ctx.piece.position,true)
}

pub fn get_possible_moves_by_knight(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(false, get_all_knight_vectors());
	helper.generate_positions(ctx.game,ctx.piece.position,false)
}