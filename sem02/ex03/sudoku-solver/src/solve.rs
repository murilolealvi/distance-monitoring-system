use std::collections::HashSet;  //sets for row and cols

pub struct SolveSudoku{
    board: Vec<Vec<char>>,
    size: usize
}

impl SolveSudoku{
    pub fn new(board: Vec<Vec<char>>, size: usize) -> Self{
        return Self{
            board,
            size
        }
    }

    pub fn board_is_valid(&self) -> bool{
        let mut row_set = HashSet::new();
        let mut col_set = HashSet::new();
        let mut box_sets = vec![HashSet::new(); self.size]; //vec! -> macro para inicializar vetores

        for row in 0..self.size{
            for col in 0..self.size{
                if let Some(digit) = self.board[row][col].to_digit(10) { //to_digit -> integer parsing by the base 10
                    //if let Some(digit) -> if let board[row][col].is_some(digit)
                    if !row_set.insert((row, digit)){ //método para HashSet == append
                        return false;
                    }
                    if !col_set.insert((col, digit)){ //se já contém o valor, retorna False
                        return false;
                    }
                    if !box_sets[3*(row/3)+col/3].insert(digit){
                        return false;
                    }
                }
            }
        }
        return true
    }

    fn guess_is_valid(&self, guess : u32) -> bool{
        let row = self.size;
        let column = self.size;
        for x in 0..self.size {
            if self.board[row][x].to_digit(10) == Some(guess) || 
            self.board[x][column].to_digit(10) == Some(guess) {
                return false;
            }
        }
        let x_index: usize = row / 3 * 3;
        let y_index: usize = column / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if self.board[x_index + x][y_index + y].to_digit(10) == Some(guess) {
                    return false;
                }
            }
        }

        return true;
    }

    fn guesses(&self) -> Vec<char> {
        let mut result = vec![];
        for guess in 1..10{
            if self.guess_is_valid(guess as u32){
                result.push(char::from_digit(guess as u32, 10).unwrap());
            }
        }
        return result
    }

    fn find_empty(&self) -> [usize;2]{
        for row in 0..self.size{
            for col in 0..self.size{
                if self.board[row][col] == '.' {
                    return [row, col];
                }
            }
        }
        return [self.size+1;2];
    }

    fn is_solved(&self) -> bool{
        self.find_empty() == [self.size+1;2]
    }

    pub fn solve(&mut self) -> Vec<Vec<char>>{
        if !self.board_is_valid() {
            return self.board;
        }

        let empty_cell: [usize;2] = self.find_empty();
        let mut result = SolveSudoku::new(self.board, self.size);

        if self.is_solved(){
            return self.board;
        }

        let row: usize = empty_cell[0];
        let col: usize = empty_cell[1];

        for guess in self.guesses() {
            result.board[row][col] = guess;
            result.board = result.solve();
            if result.is_solved() {
                return result.board;
            }
        }
        return self.board;
    }
}
