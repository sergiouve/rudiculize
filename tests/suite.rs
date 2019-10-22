use rudiculize::ridiculize;

#[test]
fn it_swaps_vowels_for_i_characters() {
    assert_eq!(ridiculize("What about loyalty?"), "Whit ibiit liyilty?");
}
