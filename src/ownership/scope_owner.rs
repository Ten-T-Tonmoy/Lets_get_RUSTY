pub fn copy_vs_move(){

    println!("------------ownership & stuffs--------------");
    //---principles------------
    //1 ) each value has a single owner (a variable)
    //2) at a time => one mutable reference ||| any number of immutable  references
    //3) when owner goes out of scope the value is dropped!?

    let mut s1=String::from("string1"); //s1 owns
    s1=String::from("changed string1"); // mutated basically or Reassignment
    // s1="new one"; not allowed must need heap
    let s2=s1; // ownership changed!
    let s1="new one"; //shadowed 
    let s3=s1; //just copy
    // s1 no longer owns shit s1 now invalid to use
    println!("so s1 is {}",s1);
    // take_away_ownerShip(s2);
    let s3=reallocate_ownership(s2);
    println!("s3 successfully got ownership of {}",s3);


    //-----------------------------moving of owenership---------------------------
    // 1) s2=s1 basic moving 
    // 2) moving by passing in a fn()
            // fn takeOwnership(s:string){} => no longer owner
    // 3) reallocation of ownership using a return fn
    // 4) move inside structs?

}

pub fn take_away_ownerShip(s:String){
    println!("Owner ship taken away of {}",s);
}

pub fn reallocate_ownership(s:String)->String{
    println!("Ownership of {} is being moved",s);
    s
}

fn main(){
    copy_vs_move();
}