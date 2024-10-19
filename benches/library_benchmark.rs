use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::fmt::Write;
use std::hint::black_box;
use std::vec::IntoIter;

use hex::BytesToHexIter;
use hex::DisplayHex;
use hex::buf_encoder::BufEncoder;
use hex::FromHex;

use hex_base::decode_to_slice;
use hex_base::decode;

use rand::RngCore;

fn display_hex(mut v: (String, Vec<u8>)) {
    write!(v.0, "{}", v.1.as_hex()).unwrap();
}

fn iter(mut v: (String, BytesToHexIter<IntoIter<u8>>)) {
    for char in v.1 {
        v.0.push(char);
    }
}

fn buf_encoder<const CAP: usize>(mut v: (BufEncoder<CAP>, Vec<u8>)) {
    v.0.put_bytes(v.1, hex::Case::Lower);
    //v.0.put_bytes(v.1);
}

fn big_from_hex_to_array(v: String) {
    let _ = <[u8; 16 * 1024]>::from_hex(&v).unwrap();
    //let mut buf = [0u8; 16 * 1024];
    //decode_to_slice(v, &mut buf).unwrap();
}

fn kib_from_hex_to_array(v: String) {
    let _ = <[u8; 1024]>::from_hex(&v).unwrap();
    //let mut buf = [0u8; 1024];
    //decode_to_slice(v, &mut buf).unwrap();
}

fn small_from_hex_to_array(v: String) {
    let _ = <[u8; 128]>::from_hex(&v).unwrap();
    //let mut buf = [0u8; 128];
    //decode_to_slice(v, &mut buf).unwrap();
}

fn from_hex_to_vec(v: String) {
    Vec::from_hex(&v).unwrap();
    //let _: Vec<u8> = decode(v).unwrap();
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

#[library_benchmark]
#[bench::first(setup = setup_big_from_hex)]
fn bench_big_from_hex_to_array(v: String) {
    black_box(big_from_hex_to_array(v))
}

#[library_benchmark]
#[bench::first(setup = setup_kib_from_hex)]
fn bench_kib_from_hex_to_array(v: String) {
    black_box(kib_from_hex_to_array(v))
}

#[library_benchmark]
#[bench::first(setup = setup_small_from_hex)]
fn bench_small_from_hex_to_array(v: String) {
    black_box(small_from_hex_to_array(v))
}

#[library_benchmark]
#[bench::small(setup = setup_small_from_hex)]
#[bench::kib(setup = setup_kib_from_hex)]
#[bench::big(setup = setup_big_from_hex)]
fn bench_from_hex_to_vec(v: String) {
    black_box(from_hex_to_vec(v))
}

library_benchmark_group!(
name = bench_encoding_group;
    benchmarks = bench_display_hex, bench_iter, bench_onekib_buf_encoder, bench_sevenkib_buf_encoder, bench_onemib_buf_encoder,
    );

library_benchmark_group!(
name = bench_decoding_group;
    benchmarks = bench_big_from_hex_to_array, bench_kib_from_hex_to_array, bench_small_from_hex_to_array, bench_from_hex_to_vec,
    );

main!(library_benchmark_groups = bench_decoding_group);

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
    let dest = BufEncoder::new();
    //let dest = BufEncoder::new(hex::Case::Lower);
    (dest, src)
}

macro_rules! setup_from_hex {
    ($LEN:expr) => {{
        let mut src = [0u8; $LEN];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut src);
        src.as_hex().to_string()
    }};
}

fn setup_big_from_hex() -> String {
    setup_from_hex!(16 * 1024)
}

fn setup_kib_from_hex() -> String {
    setup_from_hex!(1024)
}

fn setup_small_from_hex() -> String {
    setup_from_hex!(128)
}


