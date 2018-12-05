use std::io;


fn main() {

    
    println!("Which will you input, f or c?");
    let mut types = String::new();
    io::stdin().read_line(&mut types)
         .expect("failed to read line");
    println!("how much?");
    let mut number = String::new(); 
    io::stdin().read_line(&mut number)
         .expect("failed to read line");
    let number: f32 = number.trim().parse()
        .expect("Please type a number");
    let  y :f32;
    if types == "f"{
    y = (number - 32.0) / 1.8;	
    println!("result is c{}" , y);   
  } else {
    y = number * 1.8 + 32.0;
    println!("result is f{}" , y);
  }      
}
