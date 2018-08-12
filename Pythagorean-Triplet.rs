
// Pythagorean Triplet
/*

A Pythagorean triplet is a set of three natural numbers, {a, b, c}, for which,

a**2 + b**2 = c**2

For example,

3**2 + 4**2 = 9 + 16 = 25 = 5**2.

Question:
========

There exists exactly one Pythagorean triplet for which a + b + c = 1000.

Find the product a * b * c.

*/
fn main(){
    print!("{}", sam());
}

fn sam() ->  Option<u32> {
    let mut a:u32;
    let mut b:u32;
    let mut c:u32;
    let limit = 1000;
    for m in 2..limit{
        for n in 1..m{
            a = m*m - n*n;
            b = 2*m*n;
            c = m*m + n*n;

            if a+b+c == 1000{
                return Some(a*b*c);
            }
        }
    }
    None
}
