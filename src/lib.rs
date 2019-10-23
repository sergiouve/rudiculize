mod utils;

pub fn ridiculize(text: &str) -> String {
    let mut ridiculized = String::new();
    let mut iter = text.chars().peekable();
    let mut previous_character: Option<char> = None;

    while let Some(character) = iter.next() {
        let next_character: Option<&char> = iter.peek();

        if character == 'u' && previous_character.unwrap() == 'q' {
            previous_character = Some(character);
            continue;
        }

        if utils::is_vowel(&character) {
            let swapped_character = utils::swap_characters(&character, "i".to_string());
            ridiculized.push_str(&swapped_character);
            previous_character = Some(character);
            continue;
        }

        if character == 'q' && *next_character.unwrap() == 'u' {
            let swapped_character = utils::swap_characters(&character, "qu".to_string());
            ridiculized.push_str(&swapped_character);
            previous_character = Some(character);
            continue;
        }

        if character == 'c' && ! ['i', 'e'].contains(next_character.unwrap()) && utils::is_vowel(next_character.unwrap()) {
            let swapped_character = utils::swap_characters(&character, "q".to_string());
            ridiculized.push_str(&swapped_character);
            ridiculized.push_str("u");
            previous_character = Some(character);
            continue;
        }

        ridiculized.push(character);

        match next_character {
            Some(_) => continue,
            None => break,
        }
    }

    return ridiculized;
}
