

pub fn fib(n: u32) -> u32 {
	if n == 0 {
        1
    } else {
        let (res1, res2) = better(n-1);
        res1+res2
    }
}

fn better(n: u32) -> (u32,u32) {
	if n == 1 {
		(1,0)
	} else {
		println!("better({})", n-1);
		let (f1, f2) = better(n-1);
		(f1+f2, f1)
	}
}