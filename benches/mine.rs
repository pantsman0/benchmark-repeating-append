#[macro_use]
extern crate criterion;

//extern crate smallvec;
//use smallvec::SmallVec;

use criterion::Criterion;

fn extend_rle(buffer: &mut Vec<u8>, repeating_fragment_len: usize, num_bytes_to_fill: usize) {
    //clone the fragment
    let mut fragment = vec![0; repeating_fragment_len];
    fragment.extend_from_slice(&buffer[buffer.len() - repeating_fragment_len..]);

    //allocate required memory so there aren't re-allocations
    buffer.reserve(num_bytes_to_fill);

    // calculate number of full reads, and the bytes in the incomplete read
    let repeats = num_bytes_to_fill / repeating_fragment_len;
    let remainder = num_bytes_to_fill - (repeats * repeating_fragment_len);

    //repeat extension with full blocks - does nothing when num_bytes_to_fill is zero
    //panics when repeating_fragment_len is 0
    //writes up to repeating_fragment_len*floor(num_bytes_to_fill/repeating_fragment_len)
    // == num_bytes_to_fill - (num_bytes_to_fill % repeating_fragment_len)
    for _count in 0..repeats {
        buffer.extend_from_slice(fragment.as_slice());
    }
    // write the remain bytes to the buffer
    // num_bytes_to_fill - floor(num_bytes_to_fill/repeating_fragment_len)
    // == num_bytes_to_fill % repeating_fragment_len
    buffer.extend_from_slice(&fragment[0..remainder]);
}

fn copy_rle(buffer: &mut Vec<u8>, repeating_fragment_len: usize, num_bytes_to_fill: usize) {
    //clone the fragment

    //this version panics
    //let mut fragment = Vec::with_capacity(repeating_fragment_len);
    //this version is fine
    let mut fragment = vec![0; repeating_fragment_len];
    fragment.copy_from_slice(&buffer[buffer.len() - repeating_fragment_len..]);

    //allocate required memory so there aren't re-allocations
    buffer.reserve(num_bytes_to_fill);

    // calculate number of full reads, and the bytes in the incomplete read
    let repeats = num_bytes_to_fill / repeating_fragment_len;
    let remainder = num_bytes_to_fill - (repeats * repeating_fragment_len);

    //repeat extension with full blocks - does nothing when num_bytes_to_fill is zero
    //panics when repeating_fragment_len is 0
    //writes up to repeating_fragment_len*floor(num_bytes_to_fill/repeating_fragment_len)
    // == num_bytes_to_fill - (num_bytes_to_fill % repeating_fragment_len)
    for _count in 0..repeats {
        buffer.extend_from_slice(fragment.as_slice());
    }
    // write the remain bytes to the buffer
    // num_bytes_to_fill - floor(num_bytes_to_fill/repeating_fragment_len)
    // == num_bytes_to_fill % repeating_fragment_len
    buffer.extend_from_slice(&fragment[0..remainder]);
}

fn blog_func(buffer: &mut Vec<u8>, repeating_fragment_len: usize, num_bytes_to_fill: usize) {
    buffer.reserve(num_bytes_to_fill); // allocate required memory immediately, it's faster this way
    for _ in 0..num_bytes_to_fill {
        // byte_to_copy variable is needed because buffer.push(buffer[i]) doesn't compile
        let byte_to_copy = buffer[buffer.len() - repeating_fragment_len];
        buffer.push(byte_to_copy);
    }
}

fn extend_harness() {
    let mut v = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let repeat_len = 5;
    let extend_len = 5000;

    extend_rle(&mut v, repeat_len, extend_len);
}

fn copy_harness() {
    let mut v = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let repeat_len = 5;
    let extend_len = 5000;

    copy_rle(&mut v, repeat_len, extend_len);
}

fn blog_harness() {
    let mut v = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let repeat_len = 5;
    let extend_len = 5000;

    blog_func(&mut v, repeat_len, extend_len);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("extend", |b| b.iter(|| extend_harness()));
    c.bench_function("copy", |b| b.iter(|| copy_harness()));
    c.bench_function("blog", |b| b.iter(|| blog_harness()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
