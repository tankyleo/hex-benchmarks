use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::fmt::Write;
use std::hint::black_box;
use std::vec::IntoIter;

use hex::BytesToHexIter;
use hex::DisplayHex;
use hex::buf_encoder::BufEncoder;

fn display_hex(mut v: (String, Vec<u8>)) {
    write!(v.0, "{}", v.1.as_hex()).unwrap();
}

fn iter(mut v: (String, BytesToHexIter<IntoIter<u8>>)) {
    for char in v.1 {
        v.0.push(char);
    }
}

fn buf_encoder<const CAP: usize>(mut v: (BufEncoder<CAP>, Vec<u8>)) {
    v.0.put_bytes(v.1);
}

#[library_benchmark]
#[bench::sevenmib(args = (7 * 1024 * 1024), setup = setup_display_hex)]
#[bench::sevenkib(args = (7 * 1024), setup = setup_display_hex)]
#[bench::onekib(args = (1024), setup = setup_display_hex)]
#[bench::small(args = (128), setup = setup_display_hex)]
fn bench_display_hex(v: (String, Vec<u8>)) {
    black_box(display_hex(v))
}

#[library_benchmark]
#[bench::sevenmib(args = (7 * 1024 * 1024), setup = setup_iter)]
#[bench::sevenkib(args = (7 * 1024), setup = setup_iter)]
#[bench::onekib(args = (1024), setup = setup_iter)]
#[bench::small(args = (128), setup = setup_iter)]
fn bench_iter(v: (String, BytesToHexIter<IntoIter<u8>>)) {
    black_box(iter(v))
}

#[library_benchmark]
#[bench::onekib(setup = setup_buf_encoder::<1024>)]
fn bench_onekib_buf_encoder(v: (BufEncoder<1024>, Vec<u8>)) {
    black_box(buf_encoder(v))
}

#[library_benchmark]
#[bench::sevenkib(setup = setup_buf_encoder::<7168>)]
fn bench_sevenkib_buf_encoder(v: (BufEncoder<7168>, Vec<u8>)) {
    black_box(buf_encoder(v))
}

#[library_benchmark]
#[bench::onemib(setup = setup_buf_encoder::<1048576>)]
fn bench_onemib_buf_encoder(v: (BufEncoder<1048576>, Vec<u8>)) {
    black_box(buf_encoder(v))
}


library_benchmark_group!(
name = bench_encoding_group;
    benchmarks = bench_display_hex, bench_iter, bench_onekib_buf_encoder, bench_sevenkib_buf_encoder, bench_onemib_buf_encoder,
    );

main!(library_benchmark_groups = bench_encoding_group);

fn setup_display_hex(size: usize) -> (String, Vec<u8>) {
    use rand::RngCore;
    let mut src = vec![0u8; size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut src);
    let dest = String::with_capacity(2 * size);
    (dest, src)
}

fn setup_iter(size: usize) -> (String, BytesToHexIter<IntoIter<u8>>) {
    use rand::RngCore;
    let mut src = vec![0u8; size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut src);
    let dest = String::with_capacity(2 * size);
    //(dest, BytesToHexIter::new(src.into_iter()))
    (dest, BytesToHexIter::new(src.into_iter(), hex::Case::Lower))
}

fn setup_buf_encoder<const CAP: usize>() -> (BufEncoder<CAP>, Vec<u8>) {
    use rand::RngCore;
    let mut src = vec![0u8; CAP / 2];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut src);
    let dest = BufEncoder::new(hex::Case::Lower);
    (dest, src)
}

