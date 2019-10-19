use std::fmt::Display;

use crate::{Separable, SeparatorPolicy};

impl<T: Display> Separable for T {
    fn separate_by_policy(&self, policy: SeparatorPolicy) -> String {
        let original = self.to_string();
        let (before, number, after) = find_span(&original, |c| policy.digits.contains(&c));
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

    for c in number.chars().rev() {
        if Some(&counter) == groups.get(0) {
            buffer.push(sep);
            counter = 0;

            if groups.len() > 1 {
                groups = &groups[1 ..];
            }
        }

        counter += 1;
        buffer.push(c);
    }

    buffer
}

fn find_span<F>(s: &str, is_digit: F) -> (&str, &str, &str) where F: Fn(char) -> bool {
    let mut chars   = s.chars().enumerate().skip_while(|&(_, c)| !is_digit(c));

    let start       = if let Some((i, _)) = chars.next() {
        i
    } else {
        return (s, "", "");
    };

    let stop        = if let Some((i, _)) = chars.skip_while(|&(_, c)| is_digit(c)).next() {
        i
    } else {
        s.len()
    };

    (&s[.. start], &s[start .. stop], &s[stop ..])
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
