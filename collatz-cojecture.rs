pub fn collatz(n: u64) -> Option<u64> {
    let mut res:u64;
    let mut n1 = n;
    if n > 0 {
        res = 0;
        while n1 != 1 {
           if n1%2 == 0 {
                n1 = n1/2;
           }else{
                n1 = 3*n1 + 1;
           }
           res = res+1;
        }
        return Some(res);
    }
    None 
}
