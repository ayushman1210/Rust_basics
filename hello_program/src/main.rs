fn main() {
    // println!("Hello, world!");
    // let mut num:u16=256;
    //    println!("nums is {}", num);
    // num=291;
    // println!("nums is {}", num);
    // // let str:&str="Hello, Rust!";
    // // Strings are growable, so we can use the String type instead of &str
    // let mut str=String::from("Hello, Rust!");
    // str.push_str(" Welcome to programming.");
    // println!("{}", str);

//     //tupple 
//     let  mut emp_info:(&str,u32,f64)=("Alice",30,75000.0);
//     emp_info.0="Bob";
//     println!("Employee Name: {}, Age: {}, Salary: {}", emp_info.0, emp_info.1, emp_info.2);
//     let (emp_name, emp_age, emp_salary)=emp_info;
//     println!("Employee Name: {}, Age: {}, Salary: {}", emp_name, emp_age, emp_salary);
// print();
// println!("{}",add(5,10));
// let x:u8=255;
// print(x);
// let str:String=String::from("hello ayushman");
// print1(str);
// println!("the string is {}", str);

// let str1:String=get();
// println!("the string is {}", str1);
// let s2:String=String::from("world");
// let str2:String=send(s2);
// println!("the string is {}", str2);


//to avoid ownership issues we can use clone method to create a copy of the string and pass it to the function
// another way is to use tupple to return multiple values from a function and pass the ownership back to the caller
let s1:String=String::from("hello");
let len=getlen(s1.clone());
println!("the length of the {} string is  {}",s1,len);

}
fn getlen(str:String)->usize{
   let leng:usize=str.len();
   return leng;
}
// fn get()->String{
//     let new:String=String::from("hello ayushman");
//     return new;
// }

// fn send(recive:String)->String{
//     return recive;
// }
//function 
// fn print(){
//     println!("Hello, Rust!");
// }
// fn add(num1:u8, num2:u8)->u8{
//     return num1+num2;
// }

//ownership is a unique feature of Rust that ensures memory safety without needing a garbage collector. It revolves around three main rules:
//1. Each value in Rust has a variable that’s called its owner.
//2. There can only be one owner at a time.
//3. When the owner goes out of scope, the value will be dropped.
//  fn print(num:u8){
//     println!("the num is {}", num);
// }

// fn print1(item:String){
//     println!("the string is {}", item);
// }

