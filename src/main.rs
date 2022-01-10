use std  ::vec;
use Coisa::*  ;


struct  CoisaStru{
    Cor   : String,
    Number: i32
}
enum Coisa{
    Cor(String),
    Number(i32)
}
fn main() {

    //println!("Hello, world!!E meus queridos testemunhas ");
    let _num64 = 32f64;
    //let _cores = vec![12.,13.] ; 
    let _cores = vec![Cor("greem".to_string()) , Number(32)];     //let mut _cores = vec![Coisa::Cor(13),Coisa::Number(12)];

    println!("o número é {}\na cor é {}",_cores[1],_cores[0]);
    //_cores = vec![Coisa::Number(12) , Coisa::Number(12)];
    // println!("isso aqui {}",_cores[0]);
}