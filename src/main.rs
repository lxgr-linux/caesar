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
//
// key1 is not allowed to be divisible by 2 or 13
//
// by lxgr <lxgr@protonmail.com>

use std::env;
use caesar::*;


fn encrypt(nums:&Vec<u32>, s:u32, t:u32) -> Vec<u32>{
    let mut gt_nums = vec!();
    for num in nums.iter(){
        gt_nums.push(get_modulo(*num, s, t));
    }
    gt_nums
}

fn decrypt(nums:&Vec<u32>, s:u32, t:u32) -> Vec<u32>{
    let mut kt_nums = vec!();
    let mut ntn = vec!();
    for num in 0..26{
        ntn.push(get_modulo(num, s, t));
    }
    for num in nums{
        kt_nums.push(ntn.iter()
                     .position(|&r| r == *num)
                     .unwrap() as u32);
    }
    kt_nums
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
                break match &read_from_input("[E]ncrypt or [D]ecrypt?")
                                            .to_ascii_lowercase()[..]{
                    "e" => Crypt::En,
                    "d" => Crypt::De,
                    _ => continue,
                };
            }
        } else {
            match &args[1].trim().to_ascii_lowercase()[..]{
                "encrypt" => Crypt::En,
                "decrypt" => Crypt::De,
                _ => panic!("Invalid en/decryption operator")
            }
        }
    };

    let kt = {
        if args.len() == 1 {
            loop{
                // Gets text input, that should be decrypted 
                let kt = read_from_input("Enter text:")
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
                    let key = match read_from_input(&format!("Enter {} key:", text))
                                        .parse::<u32>(){
                        Ok(key) => key,
                        Err(_) => continue,
                    };
                    break {
                        // Checks if the first key is divisible by 2 or 13
                        if *text == "first" && (key % 2 == 0 || key % 13 == 0){
                            continue
                        } else {
                            key
                        }
                    };
                }
            );
        }   
    } else {
        for i in 0..2 {
            let key = args[i+2].trim()
                               .parse::<u32>()
                               .expect("Keys have to be unsigned integers!");
            key_list.push(
                // Checks if the first key is divisible by 2 or 13
                if i == 0 && (key % 2 == 0 || key % 13 == 0) {
                    panic!("The first key is not allowed to be divisible by 2 or 13");
                } else {
                    key
                }
            );
        }
    }

    let s = key_list[0];
    let t = key_list[1];
    let new_nums:Vec<u32>;

    let nums = string_to_nums(&kt);
    new_nums = match en{
            Crypt::De => decrypt(&nums, s, t),
            Crypt::En => encrypt(&nums, s, t)
    };
    println!("{}", nums_to_string(new_nums));
}
