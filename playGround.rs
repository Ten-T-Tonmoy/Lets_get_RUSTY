fn main(){

    //-----------------functionality of match--------------------
    let matchChecker:i32=3;
    match matchChecker{
        1=> println!("Yep it didnt matched"),
        2..=8 =>println!("Range based not matched"),
        9 | 10 => println!("Yep matched now"),
        _=>println!("Default unmatched to avoid panic!"),

    }

    let arr2=[5;10];
    println!("{}",arr2[3]);
}