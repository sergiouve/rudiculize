use rudiculize::ridiculize;

#[test]
fn it_swaps_vowels_for_i_characters() {
    let text = String::from("What about loyalty?");
    assert_eq!(ridiculize(&text), "Whit ibiit liyilty?");
}

#[test]
fn it_swaps_c_before_vowels_for_qu_string() {
    let text = String::from("eso me ha ofendido cantidad");
    assert_eq!(ridiculize(&text), "isi mi hi ifindidi quintidid");
}

#[test]
fn it_does_not_swap_c_for_qu_string_for_i_and_e_chars() {
    let text = String::from("Mi celebración fue la de ayer?");
    assert_eq!(ridiculize(&text), "Mi cilibriciin fii li di iyir?");
}

#[test]
fn it_maintains_qu_characters_before_vowels() {
    let text = String::from("eso que has dicho me ha ofendido cantidad");
    assert_eq!(ridiculize(&text), "isi qui his dichi mi hi ifindidi quintidid");
}

#[test]
fn it_maintains_the_characters_case() {
    let text = String::from("Imbecil, me has hecho MUCHO DAÑO");
    assert_eq!(ridiculize(&text), "Imbicil, mi his hichi MICHI DIÑI");
}
