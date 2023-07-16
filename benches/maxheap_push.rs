use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rust_heaps::MaxHeap;


fn maxheap_push_1000(c: &mut Criterion) {
    maxheap_push(c, black_box(1000));
}

fn maxheap_push_10000(c: &mut Criterion) {
    maxheap_push(c, black_box(10000));
}

fn maxheap_push(c: &mut Criterion, count: i32) {
    let mut vec_of_random_numbers = black_box(Vec::new());

    for i in 0..black_box(count) {
        vec_of_random_numbers.push(i);
    }

    let id = format!("maxheap_push({})", count);
    c.bench_function(
        id.as_str(),
        |b| b.iter(|| {
            let mut heap = black_box(MaxHeap::new());
            for value in vec_of_random_numbers.iter() {
                heap.push(value)
            }
        }),
    );
}

criterion_group!(
    benches,
    maxheap_push_1000,
    maxheap_push_10000,
);
criterion_main!(benches);
