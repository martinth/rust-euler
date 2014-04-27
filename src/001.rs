// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.
extern crate test;

use test::BenchHarness;

static BENCH_SIZE: uint = 10;

fn direct_sum(limit: uint) -> uint {
	let mut sum = 0;
	for i in range(0, limit) {
		if i % 3 == 0 ||i % 5 == 0 {
			sum += i;
		}
	}
	return sum;
}

fn using_fold(limit: uint) -> uint {
	range(0, limit)
		.filter(|n| n % 3 == 0 || n % 5 == 0)
		.fold(0, |a, b| a + b)
	
}

#[bench]
fn bench_direct_sum(b: &mut BenchHarness) {
    b.iter(|| {
        range(0, BENCH_SIZE).map(direct_sum).collect::<Vec<uint>>()
    })
}
#[bench]
fn bench_using_fold(b: &mut BenchHarness) {
    b.iter(|| {
        range(0, BENCH_SIZE).map(using_fold).collect::<Vec<uint>>()
    })
}


fn main() {
	println!("direct_sum {}", direct_sum(1000));
	println!("using_fold {}", using_fold(1000));
}