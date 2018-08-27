fn main(){
    sum_of_multiples(100, &[3,5]);
} 
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for num in 1..limit{
        if check_is_divisible(num,&factors) {
            print!("{} ",num);
            sum += num;
        }
    }
    print!("\n\n The sum is {}",sum);
    return sum;
}

fn check_is_divisible(num:u32,factors:&[u32]) -> bool {
    for factor in factors{
        if num%factor == 0{
            return true;
        }
    }
    false
}
