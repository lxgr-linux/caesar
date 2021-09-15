fn caesar(x:i32, s:i32, t:i32) -> i32{
    ((x*s)+t) % 26
}

fn string_to_nums(s:&str) -> Vec<i32>{
    let mut list = vec!();
    let abc:Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz")
                                .chars().collect();
    
    for cha in s.chars(){
        list.push(abc.iter()
                  .position(|&r| cha == r)
                  .unwrap() as i32);
    }
    list
}

fn nums_to_string(nums:Vec<i32>) -> String{
    let mut s = String::new();
    let abc:Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz")
                                .chars().collect();
    
    for num in nums.iter(){
        s.push(abc[*num as usize]);
    }
    s
}

fn main(){
    let kt = "hallowelt";
    let s = 3;
    let t = 7;
    let mut gt_nums = vec!();

    let nums = string_to_nums(&kt);
    for num in nums.iter(){
        gt_nums.push(caesar(*num, s, t));
    }
    println!("{}", nums_to_string(gt_nums));
}
