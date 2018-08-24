pub fn factors(n: u64) -> Vec<u64> {
    let mut primeFactors:Vec<u64> = Vec::new();
    let mut counter = 2;
    let mut tmpn = n;
    loop{
        if tmpn == 1{
            break;
        }
        if tmpn % counter == 0{
           primeFactors.push(counter); 
           tmpn = tmpn / counter;
        }else{
            counter = find_next_prime(counter);
        }
    }
    return primeFactors;
}

fn find_next_prime(n:u64) -> u64 {
    
    let mut num = n;
    if num == 2 {
        return 3;
    }else{
        loop{
            num = num + 2;
            if is_prime(&num) {
                return num;
            }
        }
    }

}

fn is_prime(num:&u64) -> bool{
    
    for i in 3..(*num-1){
        if num % i == 0 {
            return false;
        }
    }
    return true

}
