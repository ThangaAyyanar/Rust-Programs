
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

fn sam() -> i32 {
	let mut result:i32 = 0;
	for i in 1..1000{
    	for j in 1..1000 {
    		for k in 1..1000{
    			if i+j+k == 1000 && ( i*i + j*j ) == k*k {
    				result = i*j*k;
    			}
    		}
    	}
    }
    result
}
