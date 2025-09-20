use std::io;
use rand::Rng;
//cargo doc --open might come handy 
use std::cmp::Ordering;
use colored::*;
//asin all?

use crate::guesser::guess;

pub fn guess(){
    println!("Guess the number psych !");

    
    let mut rng=rand::rng();
    let target:u32=rng.random_range(1..=100);
    println!("The targeted random is {}",target);
    println!("Please input ur guess : ");
    // start..end operator => include start ,exclude end 
    // start..=end operator => include start and end 

    loop {
        
        let mut guessed =String::new();

        // wants a mutable reference
        io::stdin()
        .read_line(&mut  guessed)
        .expect("failed to readline");
        // expect => Result<T,E>  if error show this
        //rust makes it a number!
        let guessed:u32=match guessed.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        //shadowing .. shadows old variable 

        println!("You have guessed {} ",guessed);


        //usage of cmp returns enum . Less , Equal, Greater
        match guessed.cmp(&target) {
            //returns ordering enum
            Ordering::Less=>println!("{}","Try guessing Bigger!".red()),
            Ordering::Greater=>println!("{}","Try guessing Smaller!".yellow()),
            Ordering::Equal=>{
                println!("{}","You got me fam!".green().bold());
                break;
            },
        }
    }
    

}