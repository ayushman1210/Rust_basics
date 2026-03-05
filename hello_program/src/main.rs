// use std::io;
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
// let s1:String=String::from("hello");
// let len=getlen(s1.clone());
// println!("the length of the {} string is  {}",s1,len);

//borrow operation 
// let s1:String=String::from("hello");
// let len:usize=getlen(&s1);
// println!("the length of string is {} {}" ,len,s1)

// let mut s1:String=String::from("hello");
// append_world (&mut s1);
// println!("the string is {}", s1);
// }

// fn append_world(s3:&mut String){
// s3.push_str("world");


// let mut s1:String=String::from("hello");
// let r1=&mut s1;
// println!("the value of r1 is {}", r1);
// let r2=&mut s1;
// println!("the value of r2 is {}", r2);
// println!("the value of s1 is {}{}",r1,r2);

// refrencing and derefrencing 
// let mut x1=5;
// let y1=&mut x1;
// *y1+=1;
// println!("the value of x1 is {}", y1);

// Dangling reference 
// let ref=get();


//Data type 
// 1) scalar
// float type f32,f64
// boolean type 
// let x:bool=true;
// println!("the value of x is {}", x);
// charater type
// let char='a';
// println!("the value of char is {}", char);

//array 
// let  arr:[i32;5]=[1,2,3,4,5];
// println!("the value of arr is {}", arr.len());
// write(arr);
// vector
// let mut v:Vec<i32>=Vec::new();
// v.push(1);
// v.push(2);
// v.push(3);
// println!("the value of vector is {}", v.len());
// let v=vec![1,2,3,4,5];
// println!("the value of vector is {:?}", v);
//typeofdatatype
// let x=5;
// let y=5.0;
// println!("the type of x is {}", (&x));
// shadowing 
// conditional statement
// for loop
// let arr=[1,2,3];
// loop{
//     println!("hello")
// }

// for element in &arr{
//     println!("the value of element is {}", element);
// }
// while loop

//match 
// let num=5;

// match num {
//     1=>println!("the value of num is 1"),
//     2=>println!("the value of num is 2"),
//     3=>println!("the value of num is 3"),
//     _=>println!("the value of num is not 1,2,3")
// }

// // I/O operation
// let mut input=String::new();
// println!("enter your name");
// io::stdin().read_line(&mut input).expect("failed to read line");
// println!("hello {}", input);

}

// fn write(mut arr1:[i32;5]){
//     arr1[0]=10;
//     println!("the value of arr is {}", arr1[0]);
// }
// fn get()->&String{   
// let s:String=String::from("hello ayushman");
// return &s;
// }


// fn getlen(s1:&String)->usize{
//     return s1.len();
// }

 
// fn getlen(str:String)->usize{
//    let leng:usize=str.len();
//    return leng;
// }
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

