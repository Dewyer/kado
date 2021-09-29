use crate::chess::error::ChessError;
use crate::chess::piece::ChessPiece;
use crate::chess::game::{ChessGame, Player};

#[derive(Copy, Clone,Eq, PartialEq,Debug)]
pub struct PiecePosition
{
    pub row: usize,
    pub col: usize,
}

#[derive(Copy, Clone,Debug)]
pub struct PlayerMove
{
    pub owner:Player,
    pub from:PiecePosition,
    pub to:PiecePosition
}

pub struct PieceContext<'a,'b>
{
    pub piece:&'b ChessPiece,
    pub game:&'a ChessGame
}

impl PiecePosition
{
    pub fn new_from_cord(row: usize, col: usize) -> Result<Self,ChessError>
    {
        /*
        if  row > 7 || col > 7
        {
            return Err(ChessError::InvalidPiecePosition);
        }
        */
        Ok(Self
        {
            row,
            col
        })
    }

    fn column_letters() -> Vec<&'static str>
    {
        vec!["A","B","C","D","E","F","G","H"]
    }

    pub fn new_from_notation(notation:String) -> Result<Self,ChessError>
    {
        if notation.len() != 2
        {
            return Err(ChessError::CantParseNotation);
        }
        let cols = Self::column_letters();
        let chars = notation.chars().collect::<Vec<char>>();
        let f_pos = cols.iter().position(|cc| cc.to_string() == chars[0].to_string()).ok_or(ChessError::CantParseNotation)?;
        let s_pos:i32 = chars[1].to_string().parse().map_err(|_| ChessError::CantParseNotation)?;
        PiecePosition::new_from_cord((s_pos-1) as usize,f_pos)
    }

    pub fn add(&self,row_dif:i32,col_dif:i32) -> Result<PiecePosition,ChessError>
    {
        let new_row = self.row as i32 + row_dif;
        let new_col = self.col as i32 + col_dif;
        //println!("{} {} trying from add",new_row,new_col);

        if new_row < 0 || new_col < 0
        {
            return Err(ChessError::InvalidPiecePosition)
        }
        Self::new_from_cord( new_row as usize,new_col as usize )
    }

    pub fn get_notation(&self) -> String
    {
        format!("{}{}",Self::column_letters()[self.col],self.row+1)
    }
}

impl PlayerMove
{
    pub fn new(owner:Player,from:PiecePosition,to:PiecePosition) -> Self
    {
        Self
        {
            owner,
            from,
            to
        }
    }
}