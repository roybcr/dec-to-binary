fn main() {}

#[allow(dead_code)]
fn dec_to_bin(n: u32) -> u128 {
    let mut acc = 0u64;
    let mut powers: Vec<u64> = vec![];

    let pow2 = |p: u32| -> u64 { 2u64.pow(p) };
    let log2 = |n: u32| -> u64 { f64::log2(f64::from(n)).trunc() as u64 };

    while acc.lt(&(n as u64)) {
        let power = log2(n - acc as u32);
        acc += pow2(power as u32);
        powers.push(power);
    }

    return powers.iter().fold(0u128, |mut acc, curr| {
                            acc += 10u128.pow(*curr as u32);
                            acc
                        });
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {

    use super::dec_to_bin as dtb;

    #[test]
    fn dec_to_bin() {
        let test_group: [u32; 8] = [0, 1, 2, 5, 6, 999, 234, 658];
        let expected_results: [u128; 8] = [0, 1, 10, 101, 110, 1111100111, 11101010, 1010010010];

        for (i, _) in test_group.iter().enumerate() {
            assert_eq!(dtb(test_group[i]), expected_results[i]);
        }
    }
}
