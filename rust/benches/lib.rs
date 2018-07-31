#![feature(test)]
extern crate percent_encoding;
extern crate test;
extern crate urlquote;

use percent_encoding::{percent_encode, utf8_percent_encode, DEFAULT_ENCODE_SET};
use test::{black_box, Bencher};
use urlquote::*;

const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod\
 tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud\
 exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in\
 reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint\
 occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[bench]
fn encode_el_nino_collect_to_string(b: &mut Bencher) {
    b.iter(|| utf8_percent_encode(black_box("/El Niño/"), DEFAULT_ENCODE_SET).collect::<String>())
}

#[bench]
fn encode_lorem_ipsum_collect_to_string(b: &mut Bencher) {
    b.iter(|| utf8_percent_encode(black_box(LOREM_IPSUM), DEFAULT_ENCODE_SET).collect::<String>())
}

#[bench]
fn encode_el_nino_to_string(b: &mut Bencher) {
    b.iter(|| utf8_percent_encode(black_box("/El Niño/"), DEFAULT_ENCODE_SET).to_string())
}

#[bench]
fn encode_el_nino_one_alloc(b: &mut Bencher) {
    b.iter(|| {
        let input = black_box("/El Niño/");
        let encoded = utf8_percent_encode(input, DEFAULT_ENCODE_SET);
        let encoded_size = encoded.clone().map(str::len).sum();
        let mut buffer = String::with_capacity(encoded_size);
        for slice in encoded {
            buffer += slice
        }
    })
}

#[bench]
fn encode_4el_nino_collect(b: &mut Bencher) {
    b.iter(|| {
        utf8_percent_encode(
            black_box("/El Niño/El Niño/El Niño/El Niño/"),
            DEFAULT_ENCODE_SET,
        ).collect::<String>()
    })
}

#[bench]
fn encode_4el_nino_one_alloc(b: &mut Bencher) {
    b.iter(|| {
        let input = black_box("/El Niño/El Niño/El Niño/El Niño/");
        let encoded = utf8_percent_encode(input, DEFAULT_ENCODE_SET);
        let encoded_size = encoded.clone().map(str::len).sum();
        let mut buffer = String::with_capacity(encoded_size);
        for slice in encoded {
            buffer += slice
        }
    })
}

#[bench]
fn encode_el_nino_collect_bytes(b: &mut Bencher) {
    b.iter(|| {
        percent_encode(black_box("/El Niño/").as_bytes(), DEFAULT_ENCODE_SET).collect::<String>()
    })
}

#[bench]
fn trivial_case(b: &mut Bencher) {
    b.iter(|| {
        percent_encode(
            black_box("NoSpecialCharacters").as_bytes(),
            DEFAULT_ENCODE_SET,
        ).collect::<String>()
    })
}

#[bench]
fn quoted_len_el_nino(b: &mut Bencher) {
    let input = black_box("/El Niño/");
    b.iter(|| unsafe { quoted_len(input.as_ptr(), input.len()) })
}

#[bench]
fn quoted_len_lorem_ipsum(b: &mut Bencher) {
    let input = black_box(LOREM_IPSUM);
    b.iter(|| unsafe { quoted_len(input.as_ptr(), input.len()) })
}

#[bench]
fn c_binding_quote_el_nino(b: &mut Bencher) {
    let input = "/El Niño/";
    let len = utf8_percent_encode(input, DEFAULT_ENCODE_SET)
        .map(str::len)
        .sum();
    let mut buffer = vec![0; len];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
        )
    })
}

#[bench]
fn c_binding_quote_lorem_ipsum(b: &mut Bencher) {
    let input = LOREM_IPSUM;
    let len = utf8_percent_encode(input, DEFAULT_ENCODE_SET)
        .map(str::len)
        .sum();
    let mut buffer = vec![0; len];
    b.iter(|| unsafe {
        quote(
            input.as_ptr(),
            input.len(),
            buffer.as_mut_ptr(),
            buffer.len(),
        )
    })
}
