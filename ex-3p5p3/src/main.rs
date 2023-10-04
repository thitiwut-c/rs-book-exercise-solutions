use core::panic;

fn main() {
    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..verses.len() {
        let ordinal_day = match day {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            10 => "eleventh",
            11 => "twelfth",
            _ => panic!("Day exceeds 12"),
        };
        println!("On the {} day of Christmas", ordinal_day);
        println!("My true love sent to me");

        for gift in (0..day+1).rev() {
            println!("{}", verses[gift]);
        }

        println!();
    }
}
