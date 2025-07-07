use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use urlquote::{PYTHON_3_7_IDEMPOTENT_QUOTING, PYTHON_3_7_QUOTING};

fn python_quote_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("URL Quoting");

    // Test case 1: Already percent-encoded
    let encoded = "hello%20world%20with%20spaces";
    group.bench_function("regular-encoded", |b| {
        let mut buf = vec![0; 100];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(encoded.as_ptr()),
                black_box(encoded.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_QUOTING),
            )
        })
    });
    group.bench_function("idempotent-encoded", |b| {
        let mut buf = vec![0; 100];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(encoded.as_ptr()),
                black_box(encoded.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_IDEMPOTENT_QUOTING),
            )
        })
    });

    // Test case 2: Unencoded input
    let unencoded = "hello world with spaces";
    group.bench_function("regular-unencoded", |b| {
        let mut buf = vec![0; 100];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(unencoded.as_ptr()),
                black_box(unencoded.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_QUOTING),
            )
        })
    });
    group.bench_function("idempotent-unencoded", |b| {
        let mut buf = vec![0; 100];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(unencoded.as_ptr()),
                black_box(unencoded.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_IDEMPOTENT_QUOTING),
            )
        })
    });

    // Test case 3: Mixed content
    let mixed = "prefix%20encoded middle unencoded%20suffix";
    group.bench_function("regular-mixed", |b| {
        let mut buf = vec![0; 100];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(mixed.as_ptr()),
                black_box(mixed.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_QUOTING),
            )
        })
    });
    group.bench_function("idempotent-mixed", |b| {
        let mut buf = vec![0; 100];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(mixed.as_ptr()),
                black_box(mixed.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_IDEMPOTENT_QUOTING),
            )
        })
    });

    // Test case 4: Large input
    let large = "a very long string with spaces ".repeat(100);
    group.bench_function("regular-large", |b| {
        let mut buf = vec![0; 5000];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(large.as_ptr()),
                black_box(large.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_QUOTING),
            )
        })
    });
    group.bench_function("idempotent-large", |b| {
        let mut buf = vec![0; 5000];
        b.iter(|| unsafe {
            urlquote::quote(
                black_box(large.as_ptr()),
                black_box(large.len()),
                black_box(buf.as_mut_ptr()),
                black_box(buf.len()),
                black_box(PYTHON_3_7_IDEMPOTENT_QUOTING),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, python_quote_benchmark);
criterion_main!(benches);
