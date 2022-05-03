use std::io;


// Script to deal with expenses
//
//
// 1. Log
//      name : str
//      price: int / float
//      categorie: str
//      why? : str optional
//      reccurence? = one time | [monthly, yearly, biweekly etc]
//          -> Saves everything in a tight format to a text file
//
// 2. List 
//      show by two dates [start, end] 
//      show by month
//      show by week
//
// 3. Analyse
//      analyse by two dates
//      analyse by month
//      analyse by week
//          ---- Sum everything
//          ---- Sum by Categorie
//      

fn main() {
    let mut input = String::new();

    loop {
        println!("What did you buy today ?");
        io::stdin().read_line(&mut input).expect("Error reading input");
        println!("{}", input);
    }

}


