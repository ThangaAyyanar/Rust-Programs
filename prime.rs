pub fn nth(n: u32) -> Option<u32> {
    if n == 1 {
        return Some(2);
    }
    let mut count:u32 = 1;
    let mut number:u32 = 3;
    while count <= n {
        if is_prime(number) {
            count = count + 1;
        }
        if n == count {
            return Some(number);
        }
        number = number + 2;
    }
    None
}

fn is_prime(num: u32) -> bool{
    let mut factor:u32 = 2;
    while factor * factor <= num {
        if num % factor == 0 {
            return false;
        }
        factor = factor + 1;
    }
    true 
}
