use crate::chess::piece::piece_position::{PiecePosition, PieceContext};
use crate::chess::piece::move_helper::ChessPieceMovementHelper;

pub fn get_threatened_by_pawn(ctx:PieceContext) -> Vec<PiecePosition>
{
	let row_dif = if ctx.piece.owner.is_white() {-1} else {1};
	let helper = ChessPieceMovementHelper::new(false,vec![(row_dif,1),(row_dif,-1)]);
	helper.generate_positions(ctx.game,ctx.piece.position,true)
}

pub fn get_possible_moves_by_pawn(ctx:PieceContext) -> Vec<PiecePosition>
{
	let row_dif = if ctx.piece.owner.is_white() {-1} else {1};
	let helper = ChessPieceMovementHelper::new(false,vec![(row_dif,0)]);
	helper.generate_positions(ctx.game,ctx.piece.position,false)
}