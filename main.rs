//import standard library
use std::io;
fn main(){
    loop{
    //prompt user to enter age
    println!("Enter your age");
    let mut age_input = String::new();
    //read age 
    io::stdin()
     .read_line(&mut age_input)
     .expect("Failed to read line");
    //parse and trim
    let age:u32 = match  age_input.trim().parse(){
        Ok(age)=>age,
        Err(_)=>{
println!("enter valid age");
continue;
        }
    };
    if age>30{
        println!("you are of age");

    }
    else if age<20{
        println!("still young");
    }
    else{
        println!("Ahh so you are in your twenties");
    }
    break;

}
}