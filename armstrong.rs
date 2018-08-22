pub fn is_armstrong_number(num: u32) -> bool {
    let length = num.to_string().chars().count() as u32;
    let mut sum:u32 = 0;
    let mut tmpNum = num;
    while tmpNum != 0 {
        let digit = tmpNum % 10;
        tmpNum = tmpNum / 10;
        sum = sum + digit.pow(length);
    }
    (num == sum)
}
