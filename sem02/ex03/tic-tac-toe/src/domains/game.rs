use std::collections::VecDeque;

use super::board::Board;
use super::player::Player; //super -> parent module of the current module(same directory or code)

#[derive(Debug)]

pub struct Game{
    pub status: String,
    pub players: VecDeque<Player>, //ring stack
    pub board: Board
}

impl Game{
    pub fn new(players: VecDeque<Player>, board: Board) -> Self{
        return Self{
            players,
            board,
            status: "NOT_STARTED".to_string(), 
            //conveninência deixar vírgula para futuros incrementos
        }
    }
}