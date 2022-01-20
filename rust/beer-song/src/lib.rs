pub fn verse(n: u32) -> String {
    format!("{0} on the wall, {1}.\n{2}, {3} on the wall.\n",
            bottles(n), bottles(n).to_lowercase(), pass(n),
            bottles(if n == 0 {99} else {n - 1}).to_lowercase())
}
fn bottles(n: u32) -> String {
    match n {
        0 => "No more bottles of beer".to_string(),
        1 => "1 bottle of beer".to_string(),
        _ => format!("{} bottles of beer", n),
    }
}
fn pass(n: u32) -> &'static str {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
}
pub fn sing(last: u32, first: u32) -> String {
    (first..last + 1).rev().map(verse).collect::<Vec<_>>().join("\n")
}
