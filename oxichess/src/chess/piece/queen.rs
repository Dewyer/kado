use crate::chess::piece::piece_position::{PiecePosition, PieceContext};
use crate::chess::piece::move_helper::ChessPieceMovementHelper;
use crate::chess::piece::bishop::get_all_straight_cross;
use crate::chess::piece::rook::get_all_straight_directions;

pub fn get_all_directions()-> Vec<(i32,i32)>
{
	let bishop = get_all_straight_cross();
	let rook = get_all_straight_directions();
	let mut ff_v = Vec::new();
	for bb in bishop
	{
		ff_v.push(bb);
	}
	for rr in rook{
		ff_v.push(rr);
	}
	ff_v
}

pub fn get_threatened_by_queen(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(true, get_all_directions());
	helper.generate_positions(ctx.game,ctx.piece.position,true)
}

pub fn get_possible_moves_by_queen(ctx:PieceContext) -> Vec<PiecePosition>
{
	let helper = ChessPieceMovementHelper::new(true, get_all_directions());
	helper.generate_positions(ctx.game,ctx.piece.position,false)
}