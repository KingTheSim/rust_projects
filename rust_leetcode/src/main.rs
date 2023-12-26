fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
	if start > end {
		return 0;
	}

    let mut result = 0;
	for n in (start..=end).step_by(step as usize) {
		result += n;
	}

	result
}

fn main() {
	println!("{}", sequence_sum(1, 5, 3));
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sequence_sum;
        
    fn dotest(a: u32, b: u32, c: u32, expected: u32) {
        let actual = sequence_sum(a, b, c);
        assert!(actual == expected, 
            "With start = {a}, end = {b}, step = {c}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(2, 6, 2, 12);
        dotest(1, 5, 1, 15);
        dotest(1, 5, 3, 5);
        dotest(0, 15, 3, 45);
        dotest(16, 15, 3, 0);
        dotest(2, 24, 22, 26);
        dotest(2, 2, 2, 2);
        dotest(2, 2, 1, 2);
        dotest(1, 15, 3, 35);
        dotest(15, 1, 3, 0);
    }
}
