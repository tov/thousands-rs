use crate::SeparatorPolicy;

#[derive(Debug)]
pub struct SeparatorIterator<'a> {
    groups:                  &'a [u8],
    repeat_groups_remaining: usize,
    current_group_index:     usize,
    current_group_size:      usize,
}

impl<'a> SeparatorIterator<'a> {
    pub fn new(policy: &'a SeparatorPolicy, ndigits: usize) -> Self {
        let groups = &policy.groups;

        let mut sum = 0;

        for (index, &group) in groups.into_iter().enumerate() {
            sum += group as usize;

            if ndigits <= sum {
                return SeparatorIterator {
                    groups,
                    repeat_groups_remaining: 0,
                    current_group_index:     index,
                    current_group_size:      sum + ndigits,
                }
            }
        }

        let repeat_group_len = *groups.last()
            .expect("must provide at least one group")
            as usize;

        let digits_remaining = ndigits - sum;

        SeparatorIterator {
            groups,
            repeat_groups_remaining: digits_remaining / repeat_group_len + 1,
            current_group_index:     groups.len() - 1,
            current_group_size:      digits_remaining % repeat_group_len,
        }
    }
}

impl<'a> Iterator for SeparatorIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_group_size > 0 {
            self.current_group_size -= 1;
            return Some(false);
        }

        if self.repeat_groups_remaining > 0 {
            self.repeat_groups_remaining -= 1;
            self.current_group_size = self.groups[self.current_group_index] as usize;
            self.current_group_size -= 1;
            return Some(true);
        }

        if self.current_group_index > 0 {
            self.current_group_index -= 1;
            self.current_group_size = self.groups[self.current_group_index] as usize;
            self.current_group_size -= 1;
            return Some(true);
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use super::*;

    fn make_policy(groups: &[u8]) -> SeparatorPolicy {
        let mut result = policies::COMMA_SEPARATOR;
        result.groups = groups;
        result
    }

    fn group_string(groups: &[u8], digits: &str) -> String {
        use std::iter::once;

        let policy = &make_policy(groups);
        let iter = SeparatorIterator::new(policy, digits.len());

        iter.zip(digits.chars())
            .flat_map(|(comma, digit)|
                if comma { Some(',') } else { None }.into_iter()
                    .chain(once(digit)))
            .collect()
    }

    macro_rules! grouping_test {
        ( $name:ident, $groups:tt, $result:tt ) => {
            #[test]
            fn $name() {
                let result = $result;
                let input = $result.chars().filter(|&c| c != ',').collect::<String>();
                assert_eq!(group_string(&$groups, &input), result);
            }
        };
    }

    grouping_test!(threes_of_1, [3], "1");
    grouping_test!(threes_of_2, [3], "21");
    grouping_test!(threes_of_3, [3], "321");
    grouping_test!(threes_of_4, [3], "4,321");
    grouping_test!(threes_of_5, [3], "54,321");
    grouping_test!(threes_of_6, [3], "654,321");
    grouping_test!(threes_of_7, [3], "7,654,321");
    grouping_test!(threes_of_8, [3], "87,654,321");
    grouping_test!(threes_of_9, [3], "987,654,321");

}
