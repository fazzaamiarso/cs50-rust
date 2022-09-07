use std::io;

fn main() {
    let mut text_input = String::new();

    println!("Text: ");
    io::stdin()
        .read_line(&mut text_input)
        .expect("Failed to read line!");

    let letter_count = count_letters(&text_input);
    let word_count = count_words(&text_input);
    let sentence_count = count_sentences(&text_input);

    let read_index = calculate_index(letter_count, word_count, sentence_count);

    println!("letter: {letter_count}\n words: {word_count}\n sentences: {sentence_count}\n");
    println!("index : {read_index}");

    match read_index {
        idx if idx.ge(&16) => println!("Grade 16+"),
        idx if idx.lt(&1) => println!("Before Grade 1"),
        idx => println!("Grade {}", idx)
    }
}

fn count_words(text: &str) -> u32 {
    return text.split_whitespace().count() as u32;
}

fn count_letters(text: &str) -> u32 {
    return text.chars().filter(|c| c.is_alphanumeric()).count() as u32;
}

fn count_sentences(text: &str) -> u32 {
    return text.chars().fold(0, |acc, c| match c {
        '!' | '.' | '?' => acc + 1,
        _ => acc,
    });
}

fn calculate_index (letters : u32, words : u32, sentences : u32) -> u32 {
    let l : f32 = 100 as f32 * (letters as f32 / words as f32);
    let s : f32 = 100 as f32 * (sentences as f32 / words as f32);
    
    let index = (0.0588 * l - 0.296 * s - 15.8).round();
    return index as u32;
} 