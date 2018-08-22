pub fn square_of_sum(n: usize) -> usize {
    let result = (n*(n+1))/2;
    result*result
}

pub fn sum_of_squares(n: usize) -> usize {
    (n*(n+1)*(2*n+1))/6
}

pub fn difference(n: usize) -> usize {
    (square_of_sum(n) - sum_of_squares(n))
}
