use std::io;

fn main(){

     let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            print!("{:?}\n",check(input.as_str()));
        }
        Err(error) => println!("error: {}", error),
    }
    
}

pub fn check(candidate: &str) -> bool {
    let mut alphaVec:Vec<bool> = vec![false;26];
    for ascii in candidate.bytes(){
        let mut index = 0; 
        if ascii > 96 && ascii < 123{   
            index = (ascii - 97) as usize; 
        }else if ascii > 64 && ascii < 91 { 
            index = (ascii - 65) as usize;
        }else{
            continue;
        }
        
        print!("{}\n",index);
        
        if alphaVec[index] {
            return false;
        }else{
            alphaVec[index] = true;
        }

    }
    return true;
}
