use rudiculize::ridiculize;

#[test]
fn it_swaps_vowels_for_i_characters() {
    let text = String::from("What about loyalty?");

    assert_eq!(ridiculize(&text), "Whit ibiit liyilty?");
}
