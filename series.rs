pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series:Vec<String> = Vec::new();
    let strLen = digits.chars().count();
    if strLen > len {
        let mut i:usize = 0;
        while i+len <= strLen {
            series.push(digits[i..i+len].to_string());
            i = i+1;
        }
    }else if strLen == len {
        series.push(digits.to_string());
    }

    return series; 
}
