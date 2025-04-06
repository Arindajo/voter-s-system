//import stanard library
use std::io;

fn main(){
    //prompt user to enter age
    println!("Please enter your age");

    let mut age_input = String::new();
    //read age entered
    io::stdin()
     .read_line(&mut age_input)
     .expect("failed to read line");

    //parse and trials
    let age:u32 = match age_input.trim().parse(){
        Ok(age)=> age,
        Err(_)=>{
            println!("Please enter valid age");
            return;
        }
    };
println!("You are {} years old",age);
}