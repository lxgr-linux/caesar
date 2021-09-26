use std::io;
use std::io::Write;

pub enum Crypt{
    // Crypt enum
    En,
    De,
}

pub fn abc() -> Vec<char>{
    // Returns an alphabetic list
    String::from("abcdefghijklmnopqrstuvwxyz")
                                .chars().collect()
}

pub fn all_in_abc(kt:&str) -> bool{
    // Checks if all chars in string are actually in the alphabet
    let abc = abc();
    for cha in kt.chars(){
        if !abc.contains(&cha){
            return false;
        }
    }
    true
}

pub fn get_modulo(x:u32, s:u32, t:u32) -> u32{
    // Encrypts a number
    (x*s+t) % 26
}

pub fn string_to_nums(s:&str) -> Vec<u32>{
    // Converts a String to a Vec<u32>
    let mut list = vec!();
    let abc:Vec<char> = abc();
    
    for cha in s.chars(){
        list.push(abc.iter()
                  .position(|&r| cha == r)
                  .unwrap() as u32);
    }
    list
}

pub fn nums_to_string(nums:Vec<u32>) -> String{
    // Converts a Vec<u32> to a String
    let mut s = String::new();
    let abc:Vec<char> = abc();
    
    for num in nums.iter(){
        s.push(abc[*num as usize]);
    }
    s
}

pub fn read_from_input(text:&str) -> String{
    // Prompts the user to input a String
    let mut inp = String::new();
    print!("{} ", text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inp).unwrap();
    String::from(inp.trim())
}
