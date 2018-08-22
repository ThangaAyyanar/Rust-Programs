pub fn build_proverb(list: Vec<&str>) -> String {
    let mut resultString = String::new();
    if list.len() > 0 {

        for i in 0..(list.len()-1){

            resultString.push_str("For want of a ");
            resultString.push_str(list[i]);
            resultString.push_str(" the ");
            resultString.push_str(list[i+1]);
            resultString.push_str(" was lost.\n");

        }

        resultString.push_str("And all for the want of a ");
        resultString.push_str(list[0]);
        resultString.push_str(".");

    }
    resultString
}
