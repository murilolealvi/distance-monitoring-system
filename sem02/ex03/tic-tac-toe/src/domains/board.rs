use std::fmt;

#[derive(Debug)]

pub struct Board{
    pub size: i32,
    pub cells: Vec<Vec<char>>   //vetor/matriz no RUST
}

impl Board{ //método associado ao type(typedef)
    pub fn new(size: i32) -> Self{
        let cells: Vec<Vec<char>> = vec![vec!['-'; size as usize]; size as usize]; //cria matriz
        //operador as -> transformador de tipo (cast)
        //usize -> pointer-sized unsigned integer type
        return Self{
            size,
            cells
        }
    }

    pub fn insert_new_symbol(&mut self, row: i32, col: i32, symbol: char) -> bool{
        //board é mutável -> &mut para referenciá-lo
        if row >= self.size || col >= self.size {
            println!("linha e coluna devem ser menores que {0}...", self.size);
            return false;
        }
        if self.cells[row as usize][col as usize] != '-' {
            println!("linha e coluna estão preenchidos!");
            return false;
        }
        self.cells[row as usize][col as usize] = symbol;
        return true;
    }
}

impl fmt::Display for Board{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{ //altera formato do println
        for row in &self.cells{
            for cell in row{ //desenha o  board
                fmt.write_str(&cell.to_string())?;
                fmt.write_str(" ")?;
            }
            fmt.write_str("\n")?;
        }
        Ok(()) //EXIT_SUCCESS
    }
}