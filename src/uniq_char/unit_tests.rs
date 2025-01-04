#![allow(clippy::unwrap_used, reason = "permissible in tests")]
use assert2::assert;
use super::*;

#[test]
fn empty_input_string_returns_true() {
    /* Given */
    let expected_res = true;
    let input = "";
    let sut = uniq_char;

    /* When */
    let res = sut(input);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn single_char_ascii_input_returns_true() {
    /* Given */
    let expected_res = true;
    let input = "s";
    let sut = uniq_char;

    /* When */
    let res = sut(input);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn unique_two_char_ascii_input_returns_true() {
    /* Given */
    let expected_res = true;
    let input = "az";
    let sut = uniq_char;

    /* When */
    let res = sut(input);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn duplicate_two_char_ascii_input_returns_false() {
    /* Given */
    let expected_res = false;
    let input = "dd";
    let sut = uniq_char;

    /* When */
    let res = sut(input);

    /* Then */
    assert!(res == expected_res);
}
