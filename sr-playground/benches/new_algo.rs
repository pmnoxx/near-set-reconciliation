#[macro_use]
extern crate bencher;

use bencher::Bencher;
use sr_playground::blt::BLT;

fn create_blt(elements: impl IntoIterator<Item = u64>, capacity: usize) -> BLT {
    let mut blt = BLT::new(capacity, 11);
    for item in elements.into_iter() {
        blt.add(item);
    }
    blt
}

pub fn create_blt_bench(bench: &mut Bencher) {
    bench.iter(|| {
        let _ = create_blt(0..1000000u64, 1024);
    });
}

pub fn recover_bench_1e1_1e6(bench: &mut Bencher) {
    recover_bench(bench, 1e1 as usize, 1000000);
}

pub fn recover_bench_1e2_1e6(bench: &mut Bencher) {
    recover_bench(bench, 1e2 as usize, 1000000);
}

pub fn recover_bench_1e3_1e6(bench: &mut Bencher) {
    recover_bench(bench, 1e3 as usize, 1000000);
}

pub fn recover_bench_1e4_1e6(bench: &mut Bencher) {
    recover_bench(bench, 1e4 as usize, 1000000);
}

pub fn recover_bench_1e5_1e6(bench: &mut Bencher) {
    recover_bench(bench, 1e5 as usize, 1000000);
}

pub fn recover_bench_1e6_1e6(bench: &mut Bencher) {
    recover_bench(bench, 1e6 as usize, 1000000);
}

pub fn recover_bench(bench: &mut Bencher, count: usize, total: u64) {
    let coef = if count < 10000 { 3 } else { 3 };

    let a = create_blt(0..total, coef * count);
    let b = create_blt(
        0 + (count / 2) as u64..total + (count / 2) as u64,
        coef * count,
    );
    bench.iter(|| {
        let mut a = a.clone();
        a.merge(&b);
        let result = a.recover().unwrap();

        assert_eq!(result.len(), count);
    });
}

benchmark_group!(
    benches,
    create_blt_bench,
    recover_bench_1e1_1e6,
    recover_bench_1e2_1e6,
    recover_bench_1e3_1e6,
    recover_bench_1e4_1e6,
    //recover_bench_1e5_1e6,
    //recover_bench_1e6_1e6
);

benchmark_main!(benches);
