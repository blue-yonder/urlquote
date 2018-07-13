#![feature(test)]
extern crate percent_encoding;
extern crate test;

use percent_encoding::{utf8_percent_encode, percent_encode, DEFAULT_ENCODE_SET};

#[bench]
fn encode_el_nino_to_string(b: &mut test::Bencher) {
    b.iter(|| utf8_percent_encode(test::black_box("/El Niño/"), DEFAULT_ENCODE_SET).to_string())
}

#[bench]
fn encode_el_nino_collect(b: &mut test::Bencher) {
    b.iter(|| {
        utf8_percent_encode(test::black_box("/El Niño/"), DEFAULT_ENCODE_SET).collect::<String>()
    })
}

#[bench]
fn encode_el_nino_one_alloc(b: &mut test::Bencher) {
    b.iter(|| {
        let input = test::black_box("/El Niño/");
        let encoded = utf8_percent_encode(input, DEFAULT_ENCODE_SET);
        let encoded_size = encoded.clone().map(str::len).sum();
        let mut buffer = String::with_capacity(encoded_size);
        for slice in encoded {
            buffer += slice
        }
    })
}

#[bench]
fn encode_4el_nino_collect(b: &mut test::Bencher) {
    b.iter(|| {
        utf8_percent_encode(
            test::black_box("/El Niño/El Niño/El Niño/El Niño/"),
            DEFAULT_ENCODE_SET,
        ).collect::<String>()
    })
}

#[bench]
fn encode_4el_nino_one_alloc(b: &mut test::Bencher) {
    b.iter(|| {
        let input = test::black_box("/El Niño/El Niño/El Niño/El Niño/");
        let encoded = utf8_percent_encode(input, DEFAULT_ENCODE_SET);
        let encoded_size = encoded.clone().map(str::len).sum();
        let mut buffer = String::with_capacity(encoded_size);
        for slice in encoded {
            buffer += slice
        }
    })
}

#[bench]
fn encode_el_nino_collect_bytes(b: &mut test::Bencher) {
    b.iter(|| {
        percent_encode(
            test::black_box("/El Niño/").as_bytes(),
            DEFAULT_ENCODE_SET,
        ).collect::<String>()
    })
}

#[bench]
fn trivial_case(b: &mut test::Bencher){
    b.iter(|| {
        percent_encode(
            test::black_box("/El Niño/").as_bytes(),
            DEFAULT_ENCODE_SET,
        ).collect::<String>()
    })
}