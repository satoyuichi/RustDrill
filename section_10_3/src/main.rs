struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some year ago...");

    let first_sentence = novel.split('.').next().expect("Could not fine a '.'");
    let i = ImportantExcerpt {
	part: first_sentence,
    };
}

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

