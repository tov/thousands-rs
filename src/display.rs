use std::fmt::Display;

use crate::{Separable, SeparatorPolicy};

impl Separable for str {
    fn separate_by_policy(&self, policy: SeparatorPolicy) -> String {
        let (before, number, after, count) = find_span(&self, |c| policy.digits.contains(&c));
        let formatted = insert_separator_rev(number, policy.separator, policy.groups);

        let mut result = String::with_capacity(before.len() + formatted.len() + after.len());

        result.push_str(before);
        result.extend(formatted.chars().rev());
        result.push_str(after);

        result
    }
}

fn insert_separator_rev(number: &str, sep: char, mut groups: &[u8]) -> String {
    // Does guessing the size like on the next line make sense?
    let mut buffer  = String::with_capacity(2 * number.len());
    let mut counter = 0;

<<<<<<< HEAD
    for c in number.chars().rev() {
        if Some(&counter) == groups.get(0) {
            buffer.push(sep);
            counter = 0;

            if groups.len() > 1 {
                groups = &groups[1 ..];
=======
        for (digit, comma_after) in number.chars().zip(iter) {
            result.push(digit);
            if comma_after {
                result.push_str(policy.separator);
>>>>>>> 2d48dbd... Another UTF-8 fix; handling the empty groups case.
            }
        }

        counter += 1;
        buffer.push(c);
    }

    buffer
}

impl<T: Display> Separable for T {
    fn separate_by_policy(&self, policy: SeparatorPolicy) -> String {
        self.to_string().as_str().separate_by_policy(policy)
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
    fn smilies() {
        let policy = SeparatorPolicy {
            separator: "😃😃",
            groups:    &[1],
            digits:    &['🙁'],
        };

        assert_eq!( "  🙁🙁🙁🙁🙁  ".separate_by_policy(policy),
                    "  🙁😃😃🙁😃😃🙁😃😃🙁😃😃🙁  " );
    }

    #[test]
    fn three_two_two_two() {
        let policy = SeparatorPolicy {
            separator: ",",
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
