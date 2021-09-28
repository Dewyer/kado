pub mod king;
pub mod queen;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod move_helper;
pub mod pawn;

use crate::chess::piece::piece_position::{PiecePosition};
use crate::chess::game::{ Player};

pub mod piece_position;

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType
{
	Knight,
	Rook,
	Queen,
	King,
	Pawn,
	Bishop,
	Empty,
}

#[derive(Debug, Clone)]
pub struct ChessPiece
{
	pub owner: Player,
	pub piece_type: PieceType,
	pub position: PiecePosition,
}

impl ChessPiece
{
	pub fn new(owner: Player, piece_type: PieceType, position: PiecePosition) -> Self
	{
		Self
		{
			owner,
			piece_type,
			position,
		}
	}

	pub fn empty(position: PiecePosition) -> Self
	{
		Self
		{
			position,
			owner: Player::White,
			piece_type: PieceType::Empty,
		}
	}

	pub fn update_position(&mut self, pos:PiecePosition)
	{
		self.position = pos;
	}
}