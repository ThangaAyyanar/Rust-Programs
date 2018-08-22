pub fn verse(n: i32) -> String {
    let verse_str = n.to_string() +" "+ choose(&n)+" of beer on the wall, " +&*n.to_string() +" "+choose(&n)+ " of beer.\nTake one down and pass it around, "+ &*(n-1).to_string() + " "+choose(&(n-1))+ " of beer on the wall.\n";
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => verse_str, 
    }
}

fn choose(n:&i32) -> &'static str{ 
    match *n {
        1 => "bottle",
        _ => "bottles",
    }
}

pub fn sing(start: i32, end: i32) -> String {
        let mut finalString = String::new(); 
        let mut startmut = start;
        while startmut >= end{
            let empty_new = if startmut > end { "\n" } else { "" };
            finalString = finalString + verse(startmut).as_mut_str() +empty_new;
            startmut= startmut-1;
        }
        finalString
}
