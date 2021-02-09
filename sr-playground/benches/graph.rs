#[macro_use]
extern crate bencher;

use bencher::Bencher;

#[allow(dead_code)]
fn calculate_distance_100_100(bench: &mut Bencher) {
    bench.iter(|| {
    });
}

benchmark_group!(
    benches,
    calculate_distance_100_100
);

benchmark_main!(benches);
