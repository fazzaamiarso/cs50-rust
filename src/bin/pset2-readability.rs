use dialoguer::Input;

#[derive(Debug, PartialEq)]
struct ParsedText {
    letters: u32,
    words: u32,
    sentences: u32,
}

fn main() {
    let text_input: String = Input::new().with_prompt("Text: ").interact_text().unwrap();

    let transformed = transform_text(&text_input);

    let read_index = calculate_index(&transformed);

    println!(
        "letter: {}\n words: {}\n sentences: {}\n",
        transformed.letters, transformed.words, transformed.sentences
    );
    println!("index : {read_index}");

    match read_index {
        idx if idx.ge(&16) => println!("Grade 16+"),
        idx if idx.lt(&1) => println!("Before Grade 1"),
        idx => println!("Grade {}", idx),
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

fn transform_text(text_input: &str) -> ParsedText {
    let letters = count_letters(&text_input);
    let words = count_words(&text_input);
    let sentences = count_sentences(&text_input);

    ParsedText {
        letters,
        words,
        sentences,
    }
}

fn calculate_index(parsed: &ParsedText) -> u32 {
    let &ParsedText {
        letters,
        words,
        sentences,
    } = parsed;

    let l = 100 as f32 * (letters as f32 / words as f32);
    let s = 100 as f32 * (sentences as f32 / words as f32);

    let index = (0.0588 * l - 0.296 * s - 15.8).round();
    return index as u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform_text() {
        let text = transform_text("There are more things in Heaven and Earth, Horatio, than are dreamt of in your philosophy.");
        assert_eq!(
            text,
            ParsedText {
                letters: 72,
                words: 16,
                sentences: 1
            }
        );
    }

    #[test]
    fn test_calculate_index() {
        let transformed = transform_text("When he was nearly thirteen, my brother Jem got his arm badly broken at the elbow. When it healed, and Jem's fears of never being able to play football were assuaged, he was seldom self-conscious about his injury. His left arm was somewhat shorter than his right; when he stood or walked, the back of his hand was at right angles to his body, his thumb parallel to his thigh.");
        let reading_index = calculate_index(&transformed);

        assert_eq!(reading_index, 8);
    }
}
