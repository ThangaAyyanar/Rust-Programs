pub fn main(){
    print!("{}",square(4));
    print!("Total Grains : {}",total());
}

pub fn square(s: u32) -> u64 {

    if s > 0 && s < 65 {
        return 2u64.pow(s-1);   
    }
    else{
        panic!("Square must be between 1 and 64")
    }

}

pub fn total() -> u64 {
    
    (2u128.pow(64) - 1) as u64

}
