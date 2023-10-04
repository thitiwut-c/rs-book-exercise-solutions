use std::io;

fn main() {
    println!("Pig latin");
    println!("Enter word");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed read line");

    let word = word.trim();
    let mut pig_latin_word = String::new();
    let mut appending_word = String::from("-");

    for (i, c) in word.chars().enumerate() {
        let lower_c = c.to_lowercase().next().unwrap();

        if i == 0 {
            if lower_c == 'a'
                || lower_c == 'e'
                || lower_c == 'i'
                || lower_c == 'o'
                || lower_c == 'u'
            {
                pig_latin_word += &c.to_string();
                appending_word = format!("{appending_word}hay");
            } else {
                appending_word = format!("{appending_word}{lower_c}ay");
            }

            continue;
        }

        pig_latin_word += &c.to_string();
    }

    pig_latin_word += &appending_word;

    println!("{pig_latin_word}")
}
