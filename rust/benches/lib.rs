#![feature(test)]
extern crate percent_encoding;
extern crate test;
extern crate urlquote;

use percent_encoding::{percent_encode, utf8_percent_encode};
use test::{black_box, Bencher};
use urlquote::*;

const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod\
 tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud\
 exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in\
 reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint\
 occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[bench]
fn encode_el_nino_collect_to_string(b: &mut Bencher) {
    b.iter(|| utf8_percent_encode(black_box("/El Niño/"), DEFAULT_QUOTING).collect::<String>())
}

#[bench]
fn encode_lorem_ipsum_collect_to_string(b: &mut Bencher) {
    b.iter(|| utf8_percent_encode(black_box(LOREM_IPSUM), DEFAULT_QUOTING).collect::<String>())
}

#[bench]
fn encode_el_nino_to_string(b: &mut Bencher) {
    b.iter(|| utf8_percent_encode(black_box("/El Niño/"), DEFAULT_QUOTING).to_string())
}

#[bench]
fn trivial_case(b: &mut Bencher) {
    b.iter(|| {
        percent_encode(black_box("NoSpecialCharacters").as_bytes(), DEFAULT_QUOTING).collect::<String>()
    })
}

#[bench]
fn quoted_len_el_nino(b: &mut Bencher) {
    let input = black_box("/El Niño/");
    let mut buffer = vec![0; 1];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
            DEFAULT_QUOTING,
        )
    })
}

#[bench]
fn quoted_len_lorem_ipsum(b: &mut Bencher) {
    let input = black_box(LOREM_IPSUM);
    let mut buffer = vec![0; 1];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
            DEFAULT_QUOTING,
        )
    })
}

#[bench]
fn c_binding_quote_el_nino(b: &mut Bencher) {
    let input = "/El Niño/";
    let len = utf8_percent_encode(input, DEFAULT_QUOTING).map(str::len).sum();
    let mut buffer = vec![0; len];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
            DEFAULT_QUOTING,
        )
    })
}

#[bench]
fn c_binding_quote_lorem_ipsum(b: &mut Bencher) {
    let input = LOREM_IPSUM;
    let len = utf8_percent_encode(input, DEFAULT_QUOTING).map(str::len).sum();
    let mut buffer = vec![0; len];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
            DEFAULT_QUOTING,
        )
    })
}

#[bench]
fn c_binding_spare_space(b: &mut Bencher) {
    let input = "abc";
    let mut buffer = vec![0; 1000];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
            DEFAULT_QUOTING,
        )
    })
}
