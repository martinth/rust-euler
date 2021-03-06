// Each new term in the Fibonacci sequence is generated by adding the previous two terms. 
// By starting with 1 and 2, the first 10 terms will be:
// 		1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, 
// find the sum of the even-valued terms.


struct LimitedFibonacci {
    limit: uint,
    last: uint,
    next: uint
}

impl LimitedFibonacci {
	fn new(limit: uint) -> LimitedFibonacci {
		LimitedFibonacci {
			limit: limit,
			last: 1,
			next: 1,
		}
	}
}

impl Iterator<uint> for LimitedFibonacci {
    fn next(&mut self) -> Option<uint> {
    	let ret = self.last;
    	self.last = self.next;
    	self.next = self.last + ret;
    	if ret < self.limit {
    		Some(ret)	
    	} else {
    		None
    	}
    }
}

fn compute(limit: uint) -> uint {
	let mut fibs = LimitedFibonacci::new(limit);
	fibs.filter(|n| n % 2 == 0)
		.fold(0, |a, b| a + b)
}


fn main() {
	println!("{}", compute(4000000));
}