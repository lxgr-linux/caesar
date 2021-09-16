// A small program to de/en-crypt with the caesar procedure.
//
// Usage:
//  - For interactive use: 
// ```shell
//  $ cargo run
// ```
//
//  - For noninteractive use: 
// ```shell
//  $ cargo run <encrypt/decrypt> <key1> <key2> <text>
// ```
// by lxgr <lxgr@protonmail.com>

use std::io;
use std::io::Write;
use std::env;

fn abc() -> Vec<char>{
    // Returns the an alphabetic list
    String::from("abcdefghijklmnopqrstuvwxyz")
                                .chars().collect()
}

fn all_in_abc(kt:&str) -> bool{
    // Checks if all chars in string are actually in the alphabet
    let abc = abc();
    for cha in kt.chars(){
        if !abc.contains(&cha){
            return false;
        }
    }
    true
}

fn get_modulo(x:i32, s:i32, t:i32) -> i32{
    // Encrypts a number
    ((x*s)+t) % 26
}

fn string_to_nums(s:&str) -> Vec<i32>{
    // Converts a String to a Vec<i32>
    let mut list = vec!();
    let abc:Vec<char> = abc();
    
    for cha in s.chars(){
        list.push(abc.iter()
                  .position(|&r| cha == r)
                  .unwrap() as i32);
    }
    list
}

fn nums_to_string(nums:Vec<i32>) -> String{
    // Converts a Vec<i32> to a String
    let mut s = String::new();
    let abc:Vec<char> = abc();
    
    for num in nums.iter(){
        s.push(abc[*num as usize]);
    }
    s
}

fn encrypt(nums:&Vec<i32>, s:i32, t:i32) -> Vec<i32>{
    let mut gt_nums = vec!();
    for num in nums.iter(){
        gt_nums.push(get_modulo(*num, s, t));
    }
    gt_nums
}

fn decrypt(nums:&Vec<i32>, s:i32, t:i32) -> Vec<i32>{
    let mut kt_nums = vec!();
    let mut ntn = vec!();
    for num in 0..26{
        ntn.push(get_modulo(num, s, t));
    }
    for num in nums{
        kt_nums.push(ntn.iter()
                     .position(|&r| r == *num)
                     .unwrap() as i32);
    }
    kt_nums
}

enum Crypt{
    // Crypt enum
    En,
    De,
}

fn read_from_input(text:&str) -> String{
    let mut inp = String::new();
    print!("{} ", text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inp).unwrap();
    inp
}

fn main(){
    let args:Vec<String> = env::args().collect();
    if args.len() != 5 && args.len() != 1 {
        panic!("Error: This program needs to have 4 or 0 arguments, not {}!",
               args.len()-1);
    }

    let en:Crypt = {
        if args.len() == 1 {
            loop{
                // Gets information about process
                let en = read_from_input("[E]ncrypt or [D]ecrypt?");
                let ret = match &en.trim().to_ascii_lowercase()[..]{
                    "e" => Crypt::En,
                    "d" => Crypt::De,
                    _ => continue,
                };
                break ret
            }
        } else {
            let ret = match &args[1].trim().to_ascii_lowercase()[..]{
                "encrypt" => Crypt::En,
                "decrypt" => Crypt::De,
                _ => panic!("Invalid en/decryption operator")
            };
            ret
        }
    };

    let kt = {
        if args.len() == 1 {
            loop{
                // Gets text input, that should be decrypted 
                let kt = read_from_input("Enter text:").trim()
                                               .replace(" ", "")
                                               .to_ascii_lowercase();
                if !all_in_abc(&kt){
                continue;
                }
                break kt
            }
        } else {
            let kt = args[4].replace(" ", "").to_ascii_lowercase();
            if !all_in_abc(&kt){
                panic!("The text is only allowed to contain characters and spaces")
            }
            kt
        }
    };

    let mut key_list = vec!();
    if args.len() == 1 {
        for text in ["first", "second"].iter(){
            // Gets keys
            key_list.push(
                loop{
                    let s = read_from_input(&format!("Enter {} key:", text));
                    match s.trim().parse::<i32>(){
                        Ok(s) => break s,
                        Err(_) => continue,
                    };
                }
            );
        }   
    } else {
        for i in 0..2 {
            key_list.push(args[i+2].trim()
                                   .parse::<i32>()
                                   .expect("Keys have to be integers!")
            );
        }
    }
    let s = key_list[0];
    let t = key_list[1];
    let new_nums:Vec<i32>;

    let nums = string_to_nums(&kt);
    new_nums = match en{
            Crypt::De => decrypt(&nums, s, t),
            Crypt::En => encrypt(&nums, s, t)
    };
    println!("{}", nums_to_string(new_nums));
}
