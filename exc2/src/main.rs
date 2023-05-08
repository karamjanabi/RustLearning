//use std::io;


fn prin_sum(num1:i32 , num2:i32)
{
    let  sumit: i32 = num1 + num2;
    println!("sumit {}",sumit);

}
fn main() {
    {
        let x = 66;
        println!("{}", x);
    }

    //another_funktion();
    let number1: i32 = 64;
    let number2: i32 = 128;
    let mut sum: i32 = number1 + number2; // i added Mut for the mutable var
                                          //println!("sum : {}",number1+number2); we can write it like that but not cool you know
    sum = sum + 5;                        //.trim() delete all blanks here 
    println!("sum: {}", sum);
    prin_sum(5,5);

}
//just like robolab stuff
/* 
fn another_funktion() {
    
    let mut user_input = String::new();
    print!("Please write a number ");
    let read_result = io::stdin().read_line(&mut user_input);
    println!("user input is: {}", user_input);
    println!("user input read result: {:?}", read_result);
}

*/



