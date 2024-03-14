use std::collections::VecDeque;
use std::io;

use crate::domains::game::Game; //crate passa porções de código ao compilador != super
use crate::domains::board::Board;
use crate::domains::player::Player;

pub struct GameHandler{
    pub game: Game,
    pub moves: i32
}

impl GameHandler{
    pub fn new(players: VecDeque<Player>, board_size: i32) -> Self{
        let board: Board = Board::new(board_size); //let declara variável
        let game: Game = Game::new(players, board);

        return Self{
            game,
            moves: 0
        }
    }

    pub fn is_row_complete(&self, row: i32, symbol: char) -> bool{
        for i in 0..self.game.board.size{ //range(game.board.size)
            if self.game.board.cells[row as usize][i as usize] != symbol{
                return false;
            }
        }
        return true;
    }

    pub fn is_column_complete(&self, col: i32, symbol: char) -> bool{
        for i in 0..self.game.board.size{
            if self.game.board.cells[i as usize][col as usize] != symbol{
                return false;
            }
        }
        return true;
    }

    pub fn is_diagonal_complete(&self, symbol: char) -> bool{
        let n = self.game.board.size;
        let mut symbols_in_diagonal = 0; //mut == volatile

        for i in 0..n{
            if self.game.board.cells[i as usize][i as usize] != symbol{
                break; //começou sem símbolo já sai da condição
            }
            symbols_in_diagonal += 1; //se passar pelo if
        }
        if symbols_in_diagonal == n{
            return true //winner
        };

        symbols_in_diagonal = 0; //checar o outro lado
        for i in 0..n{
            if self.game.board.cells[i as usize][(n-i-1) as usize] != symbol{
                break;
            }
            symbols_in_diagonal += 1;
        }
        return symbols_in_diagonal == n //se True, winner
    }

    pub fn is_winner(&self, row: i32, col: i32, symbol: char) -> bool{
        return self.is_row_complete(row, symbol) ||
            self.is_column_complete(col, symbol) ||
            self.is_diagonal_complete(symbol);
    }

    pub fn change_player_turn(&mut self){
        let current_player_option = self.game.players.pop_front(); //método do VecDeque
        //retorna uma Option(Some(T(type)) or None)
        match current_player_option{ //switch/case
            Some(current_player) => {
                self.game.players.push_back(current_player);
            },
            _ => {} //qualquer outro case não realiza nenhuma operação
        }
    }
    
    pub fn get_player_input(&self) -> (i32, i32, bool){ //(linha, coluna, check)
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Dados inválidos...");
        user_input.truncate(user_input.len()-1); //remover \n no final

        let inputs: Vec<i32> = user_input.split(" ") //vetoriza o input uma vez string
        .map(|x| x.parse().expect("Não é um número...")) //parse realiza conversão de string para int
        //map itera sobre cada variável, no caso inputs, e indica uma variável temporária |x| *(lambda x:)
        .collect(); //transforma iterador em uma collection(lista)

        if inputs.len() != 2{
            println!("Digite uma linha e uma coluna separados por espaço!");
            return(0, 0, false);
        };

        return (inputs[0], inputs[1], true);
    }

    pub fn play_game(&mut self){
        while self.game.status != "COMPLETE"{
            let (mut row, mut col, is_success) = self.get_player_input();
            if !is_success{
                continue;
            }
            row -= 1;
            col -= 1;

            let game_board = &mut self.game.board;

            let is_inserted = game_board.insert_new_symbol(row, col, self.game.players[0].symbol);
            if !is_inserted{
                continue;
            }
            self.moves += 1;
            println!("{}", game_board);

            let has_won = self.is_winner(row, col, self.game.players[0].symbol);
            if has_won{
                println!("Vencedor é o player {0}", self.game.players[0].name);
                self.game.status = "COMPLETE".to_string();
                return;
            }

            let n = self.game.board.size;
            if self.moves == n*n{
                println!("O jogo terminou em empate!");
                self.game.status = "COMPLETE".to_string();
                return
            }
            self.change_player_turn();
        }
    }
}