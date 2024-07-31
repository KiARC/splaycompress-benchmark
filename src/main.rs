use std::io::Read;

use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use serde::{Deserialize, Serialize};
use serde_json;

use rayon::prelude::*;

#[derive(Debug)]
struct TestRun {
    ratio: f32,
    millis: u128,
}

fn splay_test(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    splaycompress::compress(&data[..], &mut compressed).unwrap();
    let decompressed = &mut Vec::new();
    splaycompress::decompress(&compressed[..], decompressed).unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn brotli_test(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    brotli2::read::BrotliEncoder::new(&data[..], 9)
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    brotli2::read::BrotliDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn run_test(test: fn(Vec<u8>) -> TestRun, data: Vec<Vec<u8>>) -> Vec<TestRun> {
    data.par_iter().map(|a| test(a.clone())).collect()
}

fn get_scores(tests: Vec<TestRun>) -> (f32, f32) {
    let mean_ratio: f32 =
        tests.par_iter().map(|test_run| test_run.ratio).sum::<f32>() / tests.len() as f32;
    let mean_time: f32 = tests
        .par_iter()
        .map(|test_run| test_run.millis)
        .sum::<u128>() as f32
        / tests.len() as f32;
    (mean_ratio, mean_time)
}

fn generate_data(n: u16) -> Vec<Vec<u8>> {
    (0..n)
        .into_par_iter()
        .map(|_| User::generate())
        .map(|user| serde_json::to_string(&user).unwrap())
        .map(|s| s.as_bytes().to_vec())
        .collect()
}

fn main() {
    // Generate fake data for testing
    let users = generate_data(5000);
    // Test algos
    let splay = run_test(splay_test, users.clone());
    let brotli = run_test(brotli_test, users.clone());

    // Calculate scores
    let (mean_ratio_splay, mean_time_splay) = get_scores(splay);
    let (mean_ratio_brotli, mean_time_brotli) = get_scores(brotli);

    // Print results
    println!(
        "Ran {} trials:\n==================================\nSplay:\n\tMean Ratio: {}\n\tMean Time: {}ms\n==================================\nBrotli:\n\tMean Ratio: {}\n\tMean Time: {}ms",
        users.len(),
        mean_ratio_splay,
        mean_time_splay,
        mean_ratio_brotli,
        mean_time_brotli
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    phone_number: String,
}

impl User {
    fn generate() -> Self {
        let name = Name().fake();
        let email = SafeEmail().fake();
        let phone_number = PhoneNumber().fake();
        User {
            name,
            email,
            phone_number,
        }
    }
}
