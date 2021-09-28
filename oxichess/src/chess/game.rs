use crate::chess::piece::{ChessPiece, PieceType};
use crate::chess::board_creator;
use crate::chess::error::ChessError;
use crate::chess::piece::piece_position::{PlayerMove, PiecePosition, PieceContext};
use crate::chess::piece::*;

pub type ChessBoard = Vec<Vec<ChessPiece>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CheckState
{
	NoCheck,
	White,
	Black,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameState
{
	WhiteWon,
	BlackWon,
	Draw,
	Playing,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Player
{
	Black,
	White,
}

impl Player
{
	pub fn from_bool(is_white: bool) -> Self
	{
		if is_white
		{
			Player::White
		} else {
			Player::Black
		}
	}

	pub fn is_white(&self) -> bool
	{
		match self
		{
			Player::White => true,
			Player::Black => false
		}
	}

	pub fn is_black(&self) -> bool
	{
		!self.is_white()
	}
}

#[derive(Debug, Clone)]
pub struct ChessGame
{
	pub state: GameState,
	pub on_turn: Player,
	pub board: ChessBoard,
	pub check_state: CheckState,

	white_king: PiecePosition,
	black_king: PiecePosition,
}

impl ChessGame
{
	fn update_piece_position(&mut self,old_pos:PiecePosition,new_pos:PiecePosition)
	{
		self.board[old_pos.row][old_pos.col].update_position(new_pos);
	}

	pub fn get_piece_at(&self, pos: PiecePosition) -> &ChessPiece
	{
		&self.board[pos.row][pos.col]
	}

	pub fn is_position_occupied(&self, pos: PiecePosition) -> bool
	{
		self.get_piece_at(pos).piece_type != PieceType::Empty
	}

	pub fn new() -> Self
	{
		Self
		{
			state: GameState::Playing,
			board: board_creator::construct_starter_board(),
			check_state: CheckState::NoCheck,
			on_turn: Player::White,
			white_king: PiecePosition::new_from_cord(7, 3).unwrap(),
			black_king: PiecePosition::new_from_cord(0, 4).unwrap(),
		}
	}

	fn is_position_threatened(&self, pos: PiecePosition, for_player: Player) -> bool
	{
		for row in 0..8
		{
			for col in 0..8
			{
				let at_pos = PiecePosition::new_from_cord(row, col).unwrap();
				let pp = self.get_piece_at(at_pos);
				if pp.owner != for_player
				{
					let threats = self.get_threatened_by_piece(pp);
					if threats.iter().any(|el| el == &pos)
					{
						return true;
					}
				}
			}
		}

		false
	}

	fn would_position_be_threatened(&self, pos: PiecePosition, player_move: PlayerMove) -> bool
	{
		let pseudo_game = self.make_pseudo_board(player_move);
		pseudo_game.is_position_threatened(pos, player_move.owner)
	}

	fn make_pseudo_board(&self, player_move: PlayerMove) -> ChessGame
	{
		let mut new_game = self.clone();
		new_game.execute_move(player_move);
		new_game
	}

	fn execute_move(&mut self, player_move: PlayerMove)
	{
		let piece: ChessPiece = self.board[player_move.from.row][player_move.from.col].clone();
		self.board[player_move.from.row][player_move.from.col] = ChessPiece::empty(player_move.from);
		self.board[player_move.to.row][player_move.to.col] = piece;
		self.update_piece_position(player_move.from,player_move.to);
	}

	fn validate_move(&self, player_move: &PlayerMove, moving_piece: &ChessPiece) -> Result<(), ChessError>
	{
		let playing = &self.state == &GameState::Playing;
		let owner = self.on_turn == player_move.owner;
		if !(owner && playing)
		{
			return Err(ChessError::NotRightPlayersMove);
		}

		if moving_piece.owner != self.on_turn
		{
			return Err(ChessError::PieceIsNotYours);
		}

		Ok(())
	}

	pub fn make_move(&mut self, player_move: PlayerMove) -> Result<(), ChessError>
	{
		let moving_piece = self.get_piece_at(player_move.from);

		self.validate_move(&player_move, moving_piece)?;
		let possible_moves = self.get_moves_for_piece(moving_piece);
		let is_legal_move = possible_moves.iter().any(|el| el == &player_move.to);
		if !is_legal_move
		{
			return Err(ChessError::InvalidMove);
		}

		if moving_piece.piece_type == PieceType::King
		{
			match player_move.owner
			{
				Player::White => self.white_king = player_move.to,
				Player::Black => self.black_king = player_move.to
			}
		}
		self.execute_move(player_move);
		self.check_state = self.get_new_check_state();

		self.on_turn = if self.on_turn.is_white() {Player::Black} else {Player::White};

		Ok(())
	}

	fn get_threatened_by_piece(&self, piece: &ChessPiece) -> Vec<PiecePosition>
	{
		let ctx = PieceContext { piece, game: &self };

		match &piece.piece_type
		{
			PieceType::Pawn => pawn::get_threatened_by_pawn(ctx),
			PieceType::Rook => rook::get_threatened_by_rook(ctx),
			PieceType::Bishop => bishop::get_threatened_by_bishop(ctx),
			PieceType::Knight => knight::get_threatened_by_knight(ctx),
			PieceType::King => king::get_threatened_by_king(ctx),
			PieceType::Queen => queen::get_threatened_by_queen(ctx),
			_ => vec![]
		}
	}

	fn get_possible_moves_for_piece(&self, piece: &ChessPiece) -> Vec<PiecePosition>
	{
		let ctx = PieceContext { piece, game: &self };

		match &piece.piece_type
		{
			PieceType::Pawn => pawn::get_possible_moves_by_pawn(ctx),
			PieceType::Rook => rook::get_possible_moves_by_rook(ctx),
			PieceType::Bishop => bishop::get_possible_moves_by_bishop(ctx),
			PieceType::Knight => knight::get_possible_moves_by_knight(ctx),
			PieceType::King => king::get_possible_moves_by_king(ctx),
			PieceType::Queen => queen::get_possible_moves_by_queen(ctx),
			_ => vec![]
		}
	}

	fn get_king_position_for_player(&self, player: Player) -> PiecePosition
	{
		match player
		{
			Player::Black => self.black_king,
			Player::White => self.white_king
		}
	}

	fn would_make_self_check(&self, player_move: PlayerMove) -> bool
	{
		let king_position = self.get_king_position_for_player(player_move.owner);

		self.would_position_be_threatened(king_position, player_move)
	}

	fn get_new_check_state(&self) -> CheckState
	{
		if self.is_position_threatened(self.black_king,Player::Black)
		{
			CheckState::Black
		}
		else if self.is_position_threatened(self.white_king,Player::White)
		{
			CheckState::White
		}
		else
		{
			CheckState::NoCheck
		}
	}

	pub fn get_moves_for_piece(&self, piece: &ChessPiece) -> Vec<PiecePosition>
	{
		let possible_moves = self.get_possible_moves_for_piece(piece);
		let mut filtered_moves: Vec<PiecePosition> = Vec::new();
		for p_move in possible_moves
		{
			println!("{:?} pos move.",p_move);
			let pseudo_move = PlayerMove {
				owner: piece.owner,
				from: piece.position,
				to: p_move,
			};

			if !self.would_make_self_check(pseudo_move)
			{
				filtered_moves.push(p_move);
			}
			else
			{
				println!("failed check check {:?}",pseudo_move);
			}
		}

		filtered_moves
	}
}
