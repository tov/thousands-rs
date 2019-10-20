use std::fmt::Display;

use crate::{Separable, SeparatorPolicy};
use crate::helpers::SeparatorIterator;

impl<T: Display> Separable for T {
    fn separate_by_policy(&self, policy: SeparatorPolicy) -> String {
        let original = self.to_string();
        let (before, number, after, count) = find_span(&original, |c| policy.digits.contains(&c));
        let iter = SeparatorIterator::new(&policy, count);

        let mut result = String::with_capacity(original.len() + iter.sep_len());

        result.push_str(before);

        for (digit, comma_after) in number.chars().zip(iter) {
            result.push(digit);
            if comma_after {
                result.push(policy.separator);
            }
        }

        result.push_str(after);

        result
    }
}

fn find_span<F>(s: &str, is_digit: F) -> (&str, &str, &str, usize) where F: Fn(char) -> bool {
    let number_start = match s.char_indices()
        .find_map(|(i, c)|
            if is_digit(c) {
                Some(i)
            } else {
                None
            }) {

        Some(len) => len,
        None      => return (s, "", "", 0),
    };

    let mut count = 0;

    let number_end = number_start + match s[number_start ..].char_indices()
        .find_map(|(i, c)|
            if is_digit(c) {
                count += 1;
                None
            } else {
                Some(i)
            }) {

        Some(len) => len,
        None      => s.len() - number_start,
    };

    (&s[.. number_start], &s[number_start .. number_end], &s[number_end ..], count)
}

#[cfg(test)]
mod test {
    use crate::{Separable, SeparatorPolicy, digits, policies};

    #[test]
    fn integer_thousands_commas() {
        assert_eq!( "12345".separate_with_commas(),
                    "12,345" );
    }

    #[test]
    fn three_two_two_two() {
        let policy = SeparatorPolicy {
            separator: ',',
            groups:    &[3, 2],
            digits:    &digits::ASCII_DECIMAL,
        };

        assert_eq!( "1234567890".separate_by_policy(policy),
                    "1,23,45,67,890" );
    }

    #[test]
    fn minus_sign_and_decimal_point() {
        assert_eq!( "-1234.5".separate_with_commas(),
                    "-1,234.5" );
    }

    #[test]
    fn hex_four() {
        assert_eq!( "deadbeef".separate_by_policy(policies::HEX_FOUR),
                    "dead beef" );
    }
}
