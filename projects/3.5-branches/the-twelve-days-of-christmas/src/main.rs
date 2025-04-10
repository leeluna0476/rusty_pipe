const SSIZE: usize = 12;

const NUMBERS: [&str; SSIZE] = [
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

const LYRICS: [&str; SSIZE] = [
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
    "And a partridge in a pear tree",
];

fn main() {
    for i in 0..SSIZE {
        print_ith_lyrics(i);
    }
}

fn print_ith_lyrics(i: usize) {
    println!("On the {} day of Christmas", NUMBERS[i]);
    println!("My true love sent to me");

    if i == 0 {
        println!("A {}", &LYRICS[SSIZE - 1]["And a ".len()..]);
    } else {

        for j in (0..i+1).rev() {
            println!("{}", LYRICS[SSIZE - j - 1]);
        }
    }

    if i < SSIZE - 1 {
        println!();
    }
    ss
}
