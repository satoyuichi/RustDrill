use std::collections::HashMap;

fn main() {
    let text = "hellow world wonderful world" ;

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
	let count = map.entry(word).or_insert(0);
	*count += 1;
    }

    println!("{:?}", map);
}
