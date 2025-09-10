//01 Basics
fn main() {
    let greeting : String = String ::from("Hello World ! I am Himanshu");
    println!("{}",greeting);

    variables();

    booleanl();

    str();
}

//02 Variables 
fn variables(){
    let x = 04;
    let y : i8 = 29;
    let z : u32 = 55;
    let fl : f32 = 100.01;
    println!("x: {}",x);
    println!("y: {}",y);
    println!("z: {}",z);
    println!("fl: {}",fl);

}

//03 Booleands

fn booleanl(){
    let is_male = true;
    if is_male{

        println!("You are male!");
    }else{
        println!("You are not a male!");
    }
}

//04 Strings
fn str(){
    let name: &str = "Himanshu";
    println!("Name : {}",name);
 
    let char1 = name.chars().nth(1000); 
    //println!("{}",char1); //Error

    //Rather then use this
    match char1 {
        Some(c) => println!("Character at index 1000: {}", c),
        None => println!("No character at index 1000"),
    }
}