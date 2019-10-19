fn main() {
    let text = String::from("eso que has dicho me ha ofendido cantidad");
    let mut ridiculized = String::new();
    let mut iter = text.chars().peekable();

    while let Some(character) = iter.next() {
        if iter.peek().is_none() { break; }

        let next_character = *iter.peek().unwrap();

        if character == 'q' && next_character == 'u' {
            continue;
        }

        if is_vowel(&character) {
            let swapped_character = swap_characters(&character, 'i');
            ridiculized.push_str(&swapped_character);
            continue;
        }

        // TODO
        if character == 'c' && next_character != 'i' && is_vowel(&next_character) {
            let swapped_character = swap_characters(&character, 'q');
            ridiculized.push_str(&swapped_character);
            ridiculized.push_str("u");
        }

        ridiculized.push(character);
    }

    println!("{} => {}", text, ridiculized);
}

fn is_vowel(character: &char) -> bool {
    let vowels = vec!['a', 'á', 'e', 'é', 'i', 'í', 'o', 'ó', 'u', 'ú'];
    let uppercase_vowels = vec!['A', 'Á', 'E', 'É', 'I', 'Í', 'O', 'Ó', 'U', 'Ú'];

    if vowels.contains(character) || uppercase_vowels.contains(character) {
        return true;
    }

    return false;
}

fn swap_characters(original: &char, replacement: char) -> String {
    if original.is_uppercase() {
        return replacement.to_uppercase().to_string();
    }

    return replacement.to_string();
}
