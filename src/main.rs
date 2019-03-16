mod tokenize;

use crate::tokenize::*;

fn main()
{
    let tokens = tokenize("12abc");

    println!("{:?}", tokens);
}
