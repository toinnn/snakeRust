use core::fmt;
use std::{  vec, fmt::write};
use Coisa::{Cor , Number}  ;

enum Direction{
    left ,
    right,
    up,
    down,
}
struct position{
    x  : i32,
    y  : i32
}
struct snake{
    body : Vec<position>,
    lastDirection : Direction
}
// trait Mov {
//     fn direction(&self ,dir:i8) ;
// }
impl snake {
    pub fn Next_dir(self,dir: i8) -> Direction {
    
            match dir  {
            1 => {
                snake{lastDirection:Direction::left , ..self};
                return self.lastDirection ;
            },
            2 => {
                snake{lastDirection : Direction::right , ..self};
                return self.lastDirection ;
            },
            3 => {
                snake{lastDirection : Direction::up , ..self};
                return self.lastDirection ;
            },
            4 => {
                snake{lastDirection : Direction::down , ..self};
                return self.lastDirection ;
            },
            _ => {
                format!("Tecla não prevista digitada foi : {}",dir);
                return self.lastDirection ;
            },
        }
    }
}
struct field{
    size_X:i8,
    size_y:i8,
    food_Pos:position
}
impl field {
    fn new_Food(self){
        println!("Eu gero um número aleatório entre 1 e size_x e size_y e guardo como food_Pos");        
    }
}

#[derive(Debug)]
enum Coisa{
    Cor(String),
    Number(i32)
}
impl fmt::Display for Coisa{
    fn fmt(&self ,f :&mut fmt::Formatter) -> fmt::Result{ 
        // format!();
        write!(f,"{:?}",self)
        
    }

}
fn main() {

    //println!("Hello, world!!E meus queridos testemunhas ");
    let _num64 = 32f64;
    //let _cores = vec![12.,13.] ; 
    let mut _cores = vec![Cor("greem".to_string()) , Number(32)];     //let mut _cores = vec![Coisa::Cor(13),Coisa::Number(12)];
    // let mut _cores = Cor("32".to_string());
    // let mut _stri = &"blublu"[1..];
    // println!("bla bla bla {}",_cores ); 
    // println!("{}",_stri);
    println!("o número é {}\na cor é {}",_cores[1],_cores[0]);
    //_cores = vec![Coisa::Number(12) , Coisa::Number(12)];
    // println!("isso aqui {}",_cores[0]);
}