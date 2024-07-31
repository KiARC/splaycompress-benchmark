use std::io::Read;

use fake::faker::internet::en::{Password, SafeEmail, Username};
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

fn brotli_test_good(data: Vec<u8>) -> TestRun {
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

fn brotli_test_fast(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    brotli2::read::BrotliEncoder::new(&data[..], 1)
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

fn gzip_test_good(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    flate2::read::GzEncoder::new(&data[..], flate2::Compression::best())
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    flate2::read::GzDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn gzip_test_fast(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    flate2::read::GzEncoder::new(&data[..], flate2::Compression::fast())
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    flate2::read::GzDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn zlib_test_good(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    flate2::read::ZlibEncoder::new(&data[..], flate2::Compression::best())
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    flate2::read::ZlibDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn zlib_test_fast(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    flate2::read::ZlibEncoder::new(&data[..], flate2::Compression::fast())
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    flate2::read::ZlibDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn deflate_test_good(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    flate2::read::DeflateEncoder::new(&data[..], flate2::Compression::best())
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    flate2::read::DeflateDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn deflate_test_fast(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let mut compressed: Vec<u8> = Vec::new();
    flate2::read::DeflateEncoder::new(&data[..], flate2::Compression::fast())
        .read_to_end(&mut compressed)
        .unwrap();
    let decompressed = &mut Vec::new();
    flate2::read::DeflateDecoder::new(&compressed[..])
        .read_to_end(decompressed)
        .unwrap();
    TestRun {
        ratio: compressed.len() as f32 / data.len() as f32,
        millis: start_time.elapsed().as_millis(),
    }
}

fn dummy_test(data: Vec<u8>) -> TestRun {
    let start_time = std::time::Instant::now();
    let compressed = data.clone();
    let _decompressed = compressed.clone();
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

fn mean_data_size(data: Vec<Vec<u8>>) -> f32 {
    data.par_iter().map(|a| a.len() as f32).sum::<f32>() / data.len() as f32
}

fn main() {
    // Generate fake data for testing
    let users = generate_data(10000);
    let mean_data_size = mean_data_size(users.clone());
    // Test algos
    let splay = run_test(splay_test, users.clone());
    let brotli_good = run_test(brotli_test_good, users.clone());
    let brotli_fast = run_test(brotli_test_fast, users.clone());
    let gzip_good = run_test(gzip_test_good, users.clone());
    let gzip_fast = run_test(gzip_test_fast, users.clone());
    let zlib_good = run_test(zlib_test_good, users.clone());
    let zlib_fast = run_test(zlib_test_fast, users.clone());
    let deflate_good = run_test(deflate_test_good, users.clone());
    let deflate_fast = run_test(deflate_test_fast, users.clone());
    let dummy = run_test(dummy_test, users.clone());

    // Calculate scores
    let (mean_ratio_splay, mean_time_splay) = get_scores(splay);
    let (mean_ratio_brotli_good, mean_time_brotli_good) = get_scores(brotli_good);
    let (mean_ratio_brotli_fast, mean_time_brotli_fast) = get_scores(brotli_fast);
    let (mean_ratio_gzip_good, mean_time_gzip_good) = get_scores(gzip_good);
    let (mean_ratio_gzip_fast, mean_time_gzip_fast) = get_scores(gzip_fast);
    let (mean_ratio_zlib_good, mean_time_zlib_good) = get_scores(zlib_good);
    let (mean_ratio_zlib_fast, mean_time_zlib_fast) = get_scores(zlib_fast);
    let (mean_ratio_deflate_good, mean_time_deflate_good) = get_scores(deflate_good);
    let (mean_ratio_deflate_fast, mean_time_deflate_fast) = get_scores(deflate_fast);
    let (mean_ratio_dummy, mean_time_dummy) = get_scores(dummy);

    // Print results
    println!(
        "Ran {} trials, mean data size was {} bytes:\
        \n==================================\
        \nSplay:\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nBrotli (Best Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nBrotli (Fastest Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nGzip (Best Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nGzip (Fastest Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nZLib (Best Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nZLib (Fastest Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nDeflate (Best Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nDeflate (Fastest Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms\
        \n==================================\
        \nDummy Function (No Compression):\
        \n\tMean Ratio: {}\
        \n\tMean Time: {}ms",
        users.len(),
        mean_data_size,
        mean_ratio_splay,
        mean_time_splay,
        mean_ratio_brotli_good,
        mean_time_brotli_good,
        mean_ratio_brotli_fast,
        mean_time_brotli_fast,
        mean_ratio_gzip_good,
        mean_time_gzip_good,
        mean_ratio_gzip_fast,
        mean_time_gzip_fast,
        mean_ratio_zlib_good,
        mean_time_zlib_good,
        mean_ratio_zlib_fast,
        mean_time_zlib_fast,
        mean_ratio_deflate_good,
        mean_time_deflate_good,
        mean_ratio_deflate_fast,
        mean_time_deflate_fast,
        mean_ratio_dummy,
        mean_time_dummy
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    firstname: String,
    lastname: String,
    email: String,
    phone_number: String,
    username: String,
    password: String,
}

impl User {
    fn generate() -> Self {
        let firstname = Name().fake();
        let lastname = Name().fake();
        let email = SafeEmail().fake();
        let phone_number = PhoneNumber().fake();
        let username = Username().fake();
        let password = Password(8..16).fake();

        User {
            firstname,
            lastname,
            email,
            phone_number,
            username,
            password,
        }
    }
}
