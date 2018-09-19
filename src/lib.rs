#![doc(html_root_url = "https://docs.rs/thousands/0.1.2")]
//! Provides a trait, [`Separable`], for formatting numbers with
//! separators between the digits. Typically this will be used to add
//! commas or spaces every three digits from the right, but can be
//! configured via a [`SeparatorPolicy`].
//!
//! # Examples
//!
//! The simplest way to use the library is with trait [`Separable`]’s method
//! [`separate_with_commas`] method, which does what it sounds like:
//!
//! ```
//! use thousands::Separable;
//!
//! assert_eq!(   12345  .separate_with_commas(),  "12,345" );
//! assert_eq!( (-12345) .separate_with_commas(), "-12,345" );
//! assert_eq!(    9876.5.separate_with_commas(),   "9,876.5" );
//! ```
//!
//! There are also methods [`separate_with_spaces`] and
//! [`separate_with_dots`], in case your culture uses those separators.
//!
//! However, it's also possible to pass a policy for different behavior:
//!
//! ```
//! use thousands::{Separable, SeparatorPolicy, digits};
//!
//! let policy = SeparatorPolicy {
//!     separator: ',',
//!     groups:    &[3, 2],
//!     digits:    digits::ASCII_DECIMAL,
//! };
//!
//! assert_eq!( 1234567890.separate_by_policy(policy), "1,23,45,67,890" );
//! ```
//!
//! # Usage
//!
//! It’s [on crates.io](https://crates.io/crates/thousands), so you can add
//!
//! ```toml
//! [dependencies]
//! thousands = "0.1.2"
//! ```
//!
//! to your `Cargo.toml`.
//!
//! This crate supports Rust version 1.17 and newer.
//!
//! [`Separable`]: trait.Separable.html
//! [`SeparatorPolicy`]: struct.SeparatorPolicy.html
//! [`separate_with_commas`]: trait.Separable.html#method.separate_with_commas
//! [`separate_with_spaces`]: trait.Separable.html#method.separate_with_spaces
//! [`separate_with_dots`]: trait.Separable.html#method.separate_with_dots

use std::fmt::Display;

/// Provides methods for formatting numbers with separators between the digits.
pub trait Separable {
    /// Inserts a comma every three digits from the right.
    ///
    /// This is equivalent to `self.separate_by_policy(policies::COMMA_SEPARATOR)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use thousands::*;
    /// assert_eq!( 12345.separate_with_commas(), "12,345" );
    /// ```
    fn separate_with_commas(&self) -> String {
        self.separate_by_policy(policies::COMMA_SEPARATOR)
    }

    /// Inserts a space every three digits from the right.
    ///
    /// This is equivalent to `self.separate_by_policy(policies::SPACE_SEPARATOR)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use thousands::*;
    /// assert_eq!( 12345.separate_with_spaces(), "12 345" );
    /// ```
    fn separate_with_spaces(&self) -> String {
        self.separate_by_policy(policies::SPACE_SEPARATOR)
    }

    /// Inserts a period every three digits from the right.
    ///
    /// This is equivalent to `self.separate_by_policy(policies::DOT_SEPARATOR)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use thousands::*;
    /// assert_eq!( 12345.separate_with_dots(), "12.345" );
    /// ```
    fn separate_with_dots(&self) -> String {
        self.separate_by_policy(policies::DOT_SEPARATOR)
    }

    fn separate_by_policy(&self, policy: SeparatorPolicy) -> String;
}

impl<T: Display> Separable for T {
    fn separate_by_policy(&self, policy: SeparatorPolicy) -> String {
        let original = self.to_string();
        let (before, number, after) = find_span(&original, |c| policy.digits.contains(&c));
        let formatted = insert_separator_rev(number, policy.separator, policy.groups);

        // Guessing the required size, but this will only be correct all characters in
        // `formatted` are one byte in UTF-8.
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

/// A policy for inserting separators into numbers.
///
/// The configurable aspects are:
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

/// Collections of digits.
///
/// These are used for constructing [SeparatorPolicy](struct.SeparatorPolicy.html)s.
pub mod digits {
    /// The decimal digits, in ASCII.
    pub const ASCII_DECIMAL: &[char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    /// The hexadecimal digits, in ASCII.
    pub const ASCII_HEX: &[char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
}

/// Predefined policies.
pub mod policies {
    use super::*;
    use super::digits::*;

    /// Policy for placing a comma every three decimal digits.
    pub const COMMA_SEPARATOR: SeparatorPolicy = SeparatorPolicy {
        separator:  ',',
        groups:     &[3],
        digits:     ASCII_DECIMAL,
    };

    /// Policy for placing a space every three decimal digits.
    pub const SPACE_SEPARATOR: SeparatorPolicy = SeparatorPolicy {
        separator:  ' ',
        groups:     &[3],
        digits:     ASCII_DECIMAL,
    };

    /// Policy for placing a period every three decimal digits.
    pub const DOT_SEPARATOR: SeparatorPolicy = SeparatorPolicy {
        separator:  '.',
        groups:     &[3],
        digits:     ASCII_DECIMAL,
    };

    /// Policy for placing a space every four hexadecimal digits.
    pub const HEX_FOUR: SeparatorPolicy = SeparatorPolicy {
        separator:  ' ',
        groups:     &[4],
        digits:     ASCII_HEX,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integer_thousands_commas() {
        assert_eq!( 12345.separate_with_commas(),
                    "12,345" );
    }

    #[test]
    fn three_two_two_two() {
        let policy = SeparatorPolicy {
            separator: ',',
            groups:    &[3, 2],
            digits:    &digits::ASCII_DECIMAL,
        };

        assert_eq!( 1234567890.separate_by_policy(policy),
                    "1,23,45,67,890" );
    }

    #[test]
    fn minus_sign_and_decimal_point() {
        assert_eq!( (-1234.5).separate_with_commas(),
                    "-1,234.5" );
    }

    #[test]
    fn hex_four() {
        assert_eq!( "deadbeef".separate_by_policy(policies::HEX_FOUR),
                    "dead beef" );
    }
}
