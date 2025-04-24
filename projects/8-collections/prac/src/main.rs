use std::collections::HashMap;

fn main() {
    let array = [1, 3, 2, 2, 5, 5, 5, 4, 8];

    println!("{:?}", array);
    println!("{}", get_median(&array));
    println!("{}", get_mode(&array));

    if let Some(s) = make_pig_latin("apple") {
        println!("{}", s);
    } else {
        println!("None");
    }
}

fn get_median(array: &[i32; 9]) -> i32 {
    let mut v = array.to_vec();
    v.sort();
    let mid = v.len() / 2;
    v[mid]
}

fn get_mode(array: &[i32; 9]) -> i32 {
    let mut map = HashMap::new();

    for elem in array {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }

    let mut count = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > count {
            count = value;
            mode = *key;
        }
    }

    mode
}

fn make_pig_latin(s: &str) -> Option<String> {
    let mut pig = String::new();
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = s.chars();
    let c = 
        if let Some(c) = chars.next() {
            c
        } else {
            return None;
        };

    if VOWELS.contains(&c.to_ascii_lowercase()) {
        pig.push_str(s);
        pig.push_str("-hay");
    } else {
        let rest: String = chars.collect();
        pig.push_str(&rest);
        pig.push('-');
        pig.push(c);
        pig.push_str("ay");
    }

    Some(pig)
}
