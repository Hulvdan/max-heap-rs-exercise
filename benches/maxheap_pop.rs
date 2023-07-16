use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rust_heaps::MaxHeap;


fn maxheap_pop_1000(c: &mut Criterion) {
    maxheap_pop(c, black_box(1000));
}

fn maxheap_pop_10000(c: &mut Criterion) {
    maxheap_pop(c, black_box(10000));
}

fn maxheap_pop(c: &mut Criterion, count: i32) {
    let mut vec_of_random_numbers = black_box(Vec::new());

    for i in 0..black_box(count) {
        vec_of_random_numbers.push(i);
    }

    let id = format!("maxheap_pop({})", count);
    c.bench_function(
        id.as_str(),
        |b| b.iter(|| {
            // TODO: How do I make it not to include heap's building time?..
            let mut heap = black_box(
                MaxHeap::from_vec(vec_of_random_numbers.clone())
            );

            for _ in 0..vec_of_random_numbers.len() {
                let result = heap.pop();
                black_box(result);
            }
        }),
    );
}

criterion_group!(
    benches,
    maxheap_pop_1000,
    maxheap_pop_10000,
);
criterion_main!(benches);
