use std::io;

fn main() {
    let mut input = String::new();
    
    loop {
        println!("type in a word to covert to pig latin");
        io::stdin().read_line(&mut input).unwrap();
    
        piglatinize(&mut input);
    
        println!("{}", input);

        input.clear()
    }

}

fn piglatinize(input: &mut String) {
    let initial_letter = input.remove(0);

    if is_vowel(initial_letter.to_ascii_lowercase()) {
        input.insert(0, initial_letter);
        input.push_str("-hay")
    } else {
        input.push('-');
        input.push(initial_letter);
        input.push_str("ay");
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}