use crate::chess::game::ChessGame;
use crate::chess::piece::piece_position::PiecePosition;

pub struct ChessPieceMovementHelper
{
	pub change_vectors:Vec<(i32,i32)>,
	pub repeating:bool
}

impl ChessPieceMovementHelper
{
	pub fn new(repeating:bool,change_vectors:Vec<(i32,i32)>) -> Self
	{
		Self {
			repeating,
			change_vectors
		}
	}

	pub fn generate_positions(&self,game:&ChessGame,base_position:PiecePosition,include_collision_position:bool) -> Vec<PiecePosition>
	{
		let mut results:Vec<PiecePosition> = Vec::new();
		for change_vector in self.change_vectors.iter()
		{
			let mut at_position = base_position;
			loop
			{
				if let Ok(got_position) = at_position.add(change_vector.0,change_vector.1)
				{
					let got_free = !game.is_position_occupied(got_position);
					if got_free || (!got_free && include_collision_position)
					{
						results.push(got_position);
					}
					if !got_free || !self.repeating
					{
						break;
					}

					at_position = got_position;
				}
				else
				{
					break;
				}

			}
		}

		results
	}
}