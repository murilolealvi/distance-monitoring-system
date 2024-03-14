#[derive(Debug)]

pub struct Player{ //pub makes this module visible for external modules
    pub name: String,
    pub symbol: char
}

impl Player{   //método associado ao type(typedef)
    pub fn new(name: String, symbol: char) -> Self{ //construtor com --> REFERÊNCIA 
        return Self{
            name,
            symbol
        }
    }
}