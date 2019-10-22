pub fn is_vowel(character: &char) -> bool {
    let vowels = vec!['a', 'á', 'e', 'é', 'i', 'í', 'o', 'ó', 'u', 'ú'];
    let uppercase_vowels = vec!['A', 'Á', 'E', 'É', 'I', 'Í', 'O', 'Ó', 'U', 'Ú'];

    if vowels.contains(character) || uppercase_vowels.contains(character) {
        return true;
    }

    return false;
}

pub fn swap_characters(original: &char, replacement: char) -> String {
    if original.is_uppercase() {
        return replacement.to_uppercase().to_string();
    }

    return replacement.to_string();
}
