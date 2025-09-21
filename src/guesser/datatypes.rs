
pub fn datatypes(){
    let scalary_type:u8=250; //
    let hex=0xff;
    let binary=0b1101_1001;
    let heart_eyed_cat = '😻'; 
    // i size or uSize could be an option 
    twosum(3, 5);
    compound_data_types();

}

pub fn twosum(a:i8,b:i8)->i8{
    println!("This is first arg {}",a);
    println!("This is second arg {}",b);
    println!("This is truncated like C {}",-5/3);
    // let c=a+b;
    // return c;
    // last statement auto returns nd nah no need semicolon on that
    a+b
}


pub fn compound_data_types(){
    let compound_type=(500,"its a tuple",24.5);
    let (_,affir,val)=compound_type;
    println!("This is destructed affirmation {}",affir);
    println!("This is accesed  affirmation {}",compound_type.2);
    
    //---------------------arrays----------------------

    let ar1=[1,2,3,4,100];
    let arr2=[5;10];
    println!("{}",arr2[3]);

}

pub fn array_maker(){
    let arr=[0;20];
    print!("Enter the desired size less than 20 ! ");
    let mut idx=String::new();
    


}