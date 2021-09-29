use crate::services::submission::support::{
    ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult,
    SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload,
    VerificationResult,
};
use rand::prelude::*;
use rand::{rngs::StdRng, RngCore};
use rand_distr::{Distribution, Uniform};
use std::collections::{HashMap, HashSet};
use super::rng::SeededRng;
use oxichess::chess::game::{ChessBoard, Player, ChessGame};
use oxichess::chess::piece::{ChessPiece, PieceType};
use oxichess::chess::piece::piece_position::PiecePosition;
use std::cmp::Ordering;

pub struct KingPinnedProblemSupport {}

impl KingPinnedProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KingPinnedInput {
    pub room: Vec<Vec<usize>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct KingPinnedOutput {
    pub places_to_move_to: Vec<(usize, usize)>,
}

const MIN_SIZE: usize = 8;
const MAX_SIZE: usize = 100;
pub type IRng = StdRng;

const PROBABILITY_BASE: usize = 1000;

lazy_static! {
    pub static ref PIECE_PROBABILITES: Vec<(usize, usize, usize)> = {
        vec![
            (0, 0, 200),
            (1, 200, 350),
            (2, 350, 480),
            (3, 480, 500),
            (4, 500, 580),
            (5, 580, 610),
            (0, 610, 1000),
        ]
    };
}

lazy_static! {
    pub static ref SAMPLE_INPUTS: Vec<Vec<Vec<usize>>> = {
        vec![
            vec![
                vec![0,6,0,0],
                vec![0,0,1,0],
                vec![0,1,7,0],
                vec![0,0,0,0],
            ]
        ]
    };
}

pub const CHESS_PIECES_BASE_UNICODE_VALUE:u32 = 9812;

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

pub fn display_on_console(game: &ChessGame)
{
    for row in 0..(game.board.len())
    {
        print!("|");
        for col in 0..(game.board.len())
        {
            let at_piece =  &game.board[row][col];
            print!("{}|", get_unicode_representation_of_piece(at_piece));
        }
        println!();
    }
}

fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Ordering::Equal => o2,
        _ => o1,
    }
}

impl KingPinnedProblemSupport {
    fn get_board_sizes(&self, rng: &mut SeededRng, count: usize) -> Vec<usize> {
        let mut res = vec![];

        for _ in 0..count {
            res.push(rng.gen_range(MIN_SIZE, MAX_SIZE));
        }

        res.sort();
        res
    }

    fn get_random_piece(&self, rng: &mut SeededRng) -> usize {
        let roll = rng.gen_range(0, PROBABILITY_BASE);
        PIECE_PROBABILITES.iter()
            .find(|pc|
                roll > pc.1 && roll <= pc.2
            )
            .map(|pc| pc.0)
            .unwrap_or(0)
    }

    fn generate_random_board(&self, rng: &mut SeededRng, size: usize) -> Vec<Vec<usize>> {
        let mut board = vec![];
        let king_pin_pos = (rng.gen_range(0, size), rng.gen_range(0, size));
        let mut enemy_king_pos = king_pin_pos.clone();
        while enemy_king_pos.eq(&king_pin_pos) {
            enemy_king_pos = (rng.gen_range(0, size), rng.gen_range(0, size));
        }

        for yy in 0..size {
            let mut row = vec![];
            for xx in 0..size {
                if (xx, yy).eq(&king_pin_pos) || (xx, yy).eq(&enemy_king_pos) {
                    row.push(0);
                    continue;
                }

                row.push(self.get_random_piece(rng))
            }

            board.push(row);
        }

        board[king_pin_pos.1][king_pin_pos.0] = 6;
        board[enemy_king_pos.1][enemy_king_pos.0] = 7;

        board
    }

    fn generate_input(&self, payload: SubmissionTestGenerationPayload) -> anyhow::Result<KingPinnedInput> {
        if let Some(sample) = payload.sample_index {
            return Ok(KingPinnedInput {
                room: SAMPLE_INPUTS[sample as usize].clone(),
            });
        }

        let actual_seed = if payload.sample_index.is_some() { 10 } else { payload.seed };
        let mut rng = SeededRng::new(actual_seed);

        let board_size = self.get_board_sizes(&mut rng, 6)
            .get(payload.test_index)
            .ok_or(anyhow::Error::msg("Test board size not found!"))?.clone();

        let board = self.generate_random_board(&mut rng, board_size);

        Ok(KingPinnedInput {
            room: board,
        })
    }

    fn piece_type_from_id(&self, id: usize) -> PieceType {
        match id {
            1 => PieceType::Pawn,
            2 => PieceType::Rook,
            3 => PieceType::Knight,
            4 => PieceType::Bishop,
            5 => PieceType::Queen,
            6 => PieceType::King,
            7 => PieceType::King,
            _ => PieceType::Empty,
        }
    }

    fn get_board_from_input(&self, input: &KingPinnedInput) -> (ChessBoard, PiecePosition, PiecePosition) {
        let mut brd: ChessBoard = vec![];
        let size = input.room.len();
        let mut white_king = PiecePosition::new_from_cord(0,0).unwrap();
        let mut black_king = PiecePosition::new_from_cord(0,0).unwrap();

        for yy in 0..size {
            let mut brd_row = vec![];
            for xx in 0..size {
                let piece_id = input.room[yy][xx];
                let pos = PiecePosition::new_from_cord(yy, xx).unwrap();
                let pc: ChessPiece = ChessPiece {
                    owner: if piece_id == 6 { Player::White } else { Player::Black },
                    piece_type: self.piece_type_from_id(piece_id),
                    position: pos.clone(),
                };

                if piece_id == 6 {
                    black_king = pos;
                }
                if piece_id == 7 {
                    white_king = pos;
                }

                brd_row.push(pc);
            }

            brd.push(brd_row);
        }

        (brd, white_king, black_king)
    }

    fn solver(&self, input: KingPinnedInput) -> KingPinnedOutput {
        let (board, white_king, black_king) = self.get_board_from_input(&input);
        let size = board.len();
        let chess = ChessGame::new_custom(board, white_king, black_king);
        // display_on_console(&chess);

        let mut threats: HashMap<(usize, usize), usize> = HashMap::new();

        for yy in 0..size {
            for xx in 0..size {
                threats.insert((yy, xx), 0);
            }
        }

        for yy in 0..size {
            for xx in 0..size {
                let pc = &chess.board[yy][xx];
                if pc.owner == Player::White {
                    continue;
                }

                let possible_moves = chess.get_moves_for_piece(pc);
                for move_s in possible_moves.into_iter() {
                    let pos = (move_s.col, move_s.row);
                    if threats.contains_key(&pos) {
                        let mut threat = threats.get_mut(&pos).unwrap();
                        *threat += 1;
                    } else {
                        threats.insert(pos.clone(), 1);
                    }
                }
            }
        }

        let mut threats_vec = threats.into_iter()
            .filter(|el| !chess.is_position_occupied(PiecePosition::new_from_cord(el.0.1, el.0.0).unwrap()))
            .collect::<Vec<((usize, usize), usize)>>();

        threats_vec.sort_by(|a,b| chain_ordering(a.1.cmp(&b.1), chain_ordering(a.0.0.cmp(&b.0.0), a.0.1.cmp(&b.0.1))));

        KingPinnedOutput {
            places_to_move_to: threats_vec.into_iter().map(|el| el.0).collect(),
        }
    }
}

impl ProblemSupport for KingPinnedProblemSupport {
    fn generate_submission_details(
        &self,
        payload: SubmissionGenerationPayload,
    ) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: if payload.sample_index.is_some() { 1 } else { 6 },
        })
    }

    fn generate_submission_test_input(
        &self,
        payload: SubmissionTestGenerationPayload,
    ) -> anyhow::Result<SubmissionTestGenerationResult> {
        let input = self.generate_input(payload)?;
        let input_val = serde_json::to_value(input)
            .map_err(|_| anyhow::Error::msg("Input couldn't be constructed!"))?;

        Ok(SubmissionTestGenerationResult {
            input: input_val,
            test_class: "".to_string(),
        })
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        let input = serde_json::from_str::<KingPinnedInput>(&payload.test.input)
            .map_err(|_| anyhow::Error::msg("Input couldn't be constructed!"))?;

        let solution = self.solver(input);
        let got_output = serde_json::from_value::<KingPinnedOutput>(payload.output.clone())
            .map_err(|_| anyhow::Error::msg("Output is invalid!"))?;

        println!("Correct solution: {}", serde_json::to_string(&solution).unwrap());

        Ok(VerificationResult {
            correct: solution.eq(&got_output),
        })
    }
}
