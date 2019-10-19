use crate::SeparatorPolicy;

#[derive(Debug)]
pub struct SeparatorIterator<'a> {
    groups:                  &'a [u8],
    repeat_groups_remaining: usize,
    current_group_index:     usize,
    current_group_size:      usize,
}

pub fn separate_str_iter<'a>(policy: &'a SeparatorPolicy, input: &'a str)
                             -> impl Iterator<Item = (char, bool)> + 'a {

    let iter = SeparatorIterator::new(policy, input.chars().count());
    input.chars().zip(iter)
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
                    current_group_size:      ndigits - (sum - group as usize),
                }
            }
        }

        let repeat_group_len = *groups.last()
            .expect("must provide at least one group")
            as usize;

<<<<<<< HEAD
        let len_remaining = len - sum;
        let (repeat_groups_remaining, current_group_size)
                          = ceil_div_mod(len_remaining, repeat_group_len);
=======
        let digits_remaining = ndigits - sum;
        let (repeat_groups_remaining, current_group_size)
                             = ceil_div_mod(digits_remaining, repeat_group_len);
>>>>>>> Helper for precomputing where commas go works.

        SeparatorIterator {
            groups,
            repeat_groups_remaining,
            current_group_index: groups.len() - 1,
            current_group_size,
        }
    }
}

impl<'a> Iterator for SeparatorIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_group_size.checked_sub(1).map(|current_group_size| {
            self.current_group_size = current_group_size;

            if self.current_group_size > 0 {
                return false;
            }

            if let Some(repeat_groups_remaining) = self.repeat_groups_remaining.checked_sub(1) {
                self.repeat_groups_remaining = repeat_groups_remaining;
            } else if let Some(current_group_index) = self.current_group_index.checked_sub(1) {
                self.current_group_index = current_group_index;
            } else {
                return false;
            }

            self.current_group_size = self.groups[self.current_group_index] as usize;
            return true;
        })
    }
}

fn ceil_div_mod(n: usize, m: usize) -> (usize, usize) {
    let round_up = n + m - 1;
    (round_up / m, round_up % m + 1)
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

        eprintln!("*** {:?} ***", iter);

        separate_str_iter(policy, digits)
            .flat_map(|(digit, comma_after)|
                    once(digit)
                        .chain(if comma_after { Some(',') } else { None }))
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

    grouping_test!(by_1s_of_0, [1], "");
    grouping_test!(by_1s_of_1, [1], "1");
    grouping_test!(by_1s_of_2, [1], "2,1");
    grouping_test!(by_1s_of_3, [1], "3,2,1");

    grouping_test!(by_2s_of_0, [2], "");
    grouping_test!(by_2s_of_1, [2], "1");
    grouping_test!(by_2s_of_2, [2], "21");
    grouping_test!(by_2s_of_3, [2], "3,21");
    grouping_test!(by_2s_of_4, [2], "43,21");
    grouping_test!(by_2s_of_5, [2], "5,43,21");
    grouping_test!(by_2s_of_6, [2], "65,43,21");
    grouping_test!(by_2s_of_7, [2], "7,65,43,21");
    grouping_test!(by_2s_of_8, [2], "87,65,43,21");
    grouping_test!(by_2s_of_9, [2], "9,87,65,43,21");

    grouping_test!(by_3s_of_1, [3], "1");
    grouping_test!(by_3s_of_2, [3], "21");
    grouping_test!(by_3s_of_3, [3], "321");
    grouping_test!(by_3s_of_4, [3], "4,321");
    grouping_test!(by_3s_of_5, [3], "54,321");
    grouping_test!(by_3s_of_6, [3], "654,321");
    grouping_test!(by_3s_of_7, [3], "7,654,321");
    grouping_test!(by_3s_of_8, [3], "87,654,321");
    grouping_test!(by_3s_of_9, [3], "987,654,321");

    grouping_test!(by_2s3_of_1, [3, 2], "1");
    grouping_test!(by_2s3_of_2, [3, 2], "21");
    grouping_test!(by_2s3_of_3, [3, 2], "321");
    grouping_test!(by_2s3_of_4, [3, 2], "4,321");
    grouping_test!(by_2s3_of_5, [3, 2], "54,321");
    grouping_test!(by_2s3_of_6, [3, 2], "6,54,321");
    grouping_test!(by_2s3_of_7, [3, 2], "76,54,321");
    grouping_test!(by_2s3_of_8, [3, 2], "8,76,54,321");
    grouping_test!(by_2s3_of_9, [3, 2], "98,76,54,321");

    grouping_test!(by_5s4321_of_20, [1, 2, 3, 4, 5],
                   "KJIHG,FEDCB,A987,654,32,1");
    grouping_test!(by_5s4321_of_16, [1, 2, 3, 4, 5],
                   "G,FEDCB,A987,654,32,1");
    grouping_test!(by_5s4321_of_11, [1, 2, 3, 4, 5],
                   "B,A987,654,32,1");
    grouping_test!(by_5s4321_of_10, [1, 2, 3, 4, 5],
                   "A987,654,32,1");
    grouping_test!(by_5s4321_of_9, [1, 2, 3, 4, 5],
                   "987,654,32,1");
    grouping_test!(by_5s4321_of_7, [1, 2, 3, 4, 5],
                   "7,654,32,1");
    grouping_test!(by_5s4321_of_1, [1, 2, 3, 4, 5],
                   "1");
    grouping_test!(by_5s4321_of_0, [1, 2, 3, 4, 5],
                   "");
}
