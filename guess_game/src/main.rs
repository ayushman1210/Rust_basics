use rand::Rng;
use rand::thread_rng;
use std::io;
fn main() {
  let fruits=vec!["apple","banana","orange","papaya", "grapes", "watermelon", "kiwi", "mango", "pineapple", "strawberry", "blueberry", "raspberry", "blackberry", "peach", "pear", "plum", "cherry", "apricot", "nectarine", "coconut", "fig", "pomegranate", "guava", "lychee", "passionfruit", "dragonfruit", "durian", "jackfruit", "tamarind", "starfruit", "persimmon", "cranberry"];
     let mut rng = thread_rng();
    let inx=rng.gen_range(0..fruits.len());
    let random=fruits[inx];
    println!("please guess the fruit !!");
    let mut input=String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) =>{
            let fruit_selected=input.trim().to_lowercase();
            if fruit_selected==random{
                println!("Congratulations! You guessed the fruit correctly: {}", random);
            }
            else{
                println!("Sorry, the correct fruit was: {}", random);
        }
    }
        Err(e)=>{println!("Error reading input: {}",e);}
    }
   
}
