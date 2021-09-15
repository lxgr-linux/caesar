fn caesar(x:i32, s:i32, t:i32) -> i32{
    ((x*s)+t) % 26
}

fn main(){
    let ls = [10, 0, 13, 13];
    let s = 3;
    let t = 7;

    for x in ls.iter(){
        println!("{}", caesar(*x, s, t))
    }   
}
