//! Provides a type, [Separated](struct.Separated.html) for printing numbers with
//!
//! digits, but can be configured via a [SeparatorPolicy](struct.SeparatorPolicy.html).

use std::fmt::{Display, Formatter, Write, Result};

/// Wrapper struct for printing numbers with separators.
///
/// # Examples
///
/// ```rust
///
/// ```
#[derive(Clone)]
pub struct Separated<'a, T> {
    value:  T,
    policy: SeparatorPolicy<'a>,
}

impl<'a, T> Separated<'a, T> {
    /// Creates a wrapper object for printing the given value using the given
    /// separator policy.
    pub fn with_policy(value: T, policy: SeparatorPolicy<'a>) -> Self {
        Separated { value, policy }
    }

    /// Creates a wrapper object for printing the given value with a comma
    /// every three digits (from the right). This is equivalent to
    ///
    /// ```
    /// Separated::with_policy(value, COMMA_SEPARATOR_POLICY)
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!( Separated::commas(1234567).to_string(),
    ///             "1,234,567" );
    /// ```
    pub fn commas(value: T) -> Self {
        Separated::with_policy(value, COMMA_SEPARATOR_POLICY)
    }

    /// Creates a wrapper object for printing the given value with a space
    /// every three digits (from the right). This is equivalent to
    ///
    /// ```
    /// Separated::with_policy(value, SPACE_SEPARATOR_POLICY)
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!( Separated::spaces(1234567).to_string(),
    ///             "1 234 567" );
    /// ```
    pub fn spaces(value: T) -> Self {
        Separated::with_policy(value, SPACE_SEPARATOR_POLICY)
    }

    /// Extracts the wrapped value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<'a, T: Display> Display for Separated<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let original = self.value.to_string();
        let (before, number, after) = find_span(&original, |c| self.policy.digits.contains(&c));
        let formatted = insert_separator(number, self.policy.separator, self.policy.groups);

        f.write_str(before)?;
        for c in formatted.into_iter() {
            f.write_char(c)?;
        }
        f.write_str(after)?;

        Ok(())
    }
}

fn insert_separator(number: &str, sep: char, mut groups: &[u8]) -> Vec<char> {
    let mut buffer  = Vec::with_capacity(2 * number.len());
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

    buffer.reverse();

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

/// A policy for inserting separators into numbers. The configurable aspects are:
///
///   - The separator character to insert.
///
///   - How to group the separators.
///
///   - What characters are considered digits (for skipping non-digits such as
///     a minus sign).
#[derive(Debug, Clone, Copy)]
pub struct SeparatorPolicy<'a> {
    /// The separator to insert.
    pub separator: char,
    /// The grouping. The numbers in this array give the size of the groups, from
    /// right to left, with the last number in the array giving the size of all
    /// subsequent groups.
    ///
    /// So to group by threes, as is typical in many places,
    /// this array should be `&[3]`. However, to get a grouping like `1,23,45,678`,
    /// where the last group has size three and the others size two, you would use
    /// `&[3, 2]`.
    pub groups:    &'a [u8],
    /// The characters that are considered digits. If there are multiple groups of
    /// digits separated by non-digits, we only add separators to the first group.
    /// This means, for example, that the number `-12345.67` will only have separators
    /// inserted into the `12345` portion.
    pub digits:    &'a [char],
}

/// The decimal digits, in ASCII.
pub const ASCII_DECIMAL_DIGITS: &'static [char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

/// The hexadecimal digits, in ASCII.
pub const ASCII_HEX_DIGITS: &'static [char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Policy for placing a comma every three decimal digits.
pub const COMMA_SEPARATOR_POLICY: SeparatorPolicy<'static> = SeparatorPolicy {
    separator:  ',',
    groups:     &[3],
    digits:     ASCII_DECIMAL_DIGITS,
};

/// Policy for placing a space every three decimal digits.
pub const SPACE_SEPARATOR_POLICY: SeparatorPolicy<'static> = SeparatorPolicy {
    separator:  ' ',
    groups:     &[3],
    digits:     ASCII_DECIMAL_DIGITS,
};

/// Policy for placing a space every four hexadecimal digits.
pub const HEX_FOUR_POLICY: SeparatorPolicy<'static> = SeparatorPolicy {
    separator:  ' ',
    groups:     &[4],
    digits:     ASCII_HEX_DIGITS,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integer_thousands_commas() {
        assert_eq!( Separated::commas(12345).to_string(),
                    "12,345" );
    }

    #[test]
    fn three_two_two_two() {
        let policy = SeparatorPolicy {
            separator: ',',
            groups:    &[3, 2],
            digits:    &ASCII_DECIMAL_DIGITS,
        };

        assert_eq!( Separated::with_policy(1234567890, policy).to_string(),
                    "1,23,45,67,890" );
    }

    #[test]
    fn minus_sign_and_decimal_point() {
        assert_eq!( Separated::commas(-1234.5).to_string(),
                    "-1,234.5" );
    }

    #[test]
    fn hex_four() {
        assert_eq!( Separated::with_policy("deadbeef", HEX_FOUR_POLICY).to_string(),
                    "dead beef" );
    }
}
