pub fn raindrops(n: usize) -> String {
    let mut finalString:String = "".to_string();
    if n % 3 == 0 {
        finalString += "Pling";
    }
    if n % 5 == 0 {
        finalString += "Plang";
    }
    if n % 7 == 0 {
        finalString += "Plong";
    }
    if finalString == "" {
        finalString = n.to_string();
    }
    finalString
}
