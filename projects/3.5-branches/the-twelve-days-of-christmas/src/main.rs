const SSIZE: usize = 12;

fn main() {
    let numbers: [&str; SSIZE] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "11th",
        "12th",
    ];

    let lyrics: [&str; SSIZE] = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtle-doves",
        "partridge in a pear tree",
    ];

    print_lyrics(0, &numbers, &lyrics);
    for i in 1..SSIZE {
        print_lyrics(i, &numbers, &lyrics);
    }
}

fn print_lyrics(i: usize, numbers: & [&str; SSIZE], lyrics: & [&str; SSIZE]) {
    println!("On the {} day of Chrstmas", numbers[i]);
    println!("My true love sent to me");

    for j in (0..i+1).rev() {
        if j == 0 {
            print!("{}", if i == 0 { "A " } else { "And a " });
        }
        println!("{}", lyrics[SSIZE - j - 1]);
    }

    if i < SSIZE - 1 {
        println!();
    }
}
