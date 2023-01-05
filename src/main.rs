use std::{env::args, io, io::prelude::*};

fn main() {
      let Some(arg) = args().nth(1) else { return };
      let arg: u64 = arg.as_str().parse().expect(
            format!(
                  "The provided argument must be a positive integer between 0 - {}",
                  u64::MAX
            )
            .as_str(),
      );

      writeln!(io::stdout(), "{}", d2b(arg)).expect("Failed to write to stdout!");
}

fn d2b(n: u64) -> String {
      if n == 0 {
            return "0".to_string();
      }
      let mut n = n;
      let mut bin = String::new();
      while n > 0 {
            bin.push_str(&(n % 2).to_string()[..]);
            n /= 2;
      }

      bin.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {

      #[test]
      fn td2b() {
            let test_group: [u64; 8] = [0, 1, 2, 5, 6, 999, 234, 658];
            let expected_results: [&'static str; 8] = [
                  "0",
                  "1",
                  "10",
                  "101",
                  "110",
                  "1111100111",
                  "11101010",
                  "1010010010",
            ];

            for (i, _) in test_group.iter().enumerate() {
                  assert_eq!(super::d2b(test_group[i]), expected_results[i]);
            }
      }
}
