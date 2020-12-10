/// Given an input string which is a newline delimited list of numbers, locate
/// two numbers which sum to `sum_target` and return their values multiplied
/// together.
#[allow(dead_code)]
fn day_1_part_1(input: &str, sum_target: i32) -> Option<i32> {
    let mut nums = day_1_parse(input);
    nums.sort_unstable();

    day_1_find_two_numbers_summing_to(&nums, sum_target)
}

/// Given an input string which is a newline delimited list of numbers, locate
/// three numbers which sum to `sum_target` and return their values multiplied
/// together.
#[allow(dead_code)]
fn day_1_part_2(input: &str, sum_target: i32) -> Option<i32> {
    let mut nums = day_1_parse(input);
    nums.sort_unstable();

    for (idx, a) in nums.iter().enumerate() {
        if let Some(value) = day_1_find_two_numbers_summing_to(&nums[idx + 1..], sum_target - a) {
            return Some(value * a);
        }
    }

    None
}

fn day_1_parse(input: &str) -> Vec<i32> {
    input.lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", x))
        })
        .collect()
}

/// Algorithm here will look one number at a time, subtract it from
/// `sum_target`, and then look at each number starting from the end of the
/// array to see if there is a number equal to this.
///
/// Each loop is made faster by continuing the search for the bigger number
/// from the index we looked at last in the previous iteration (since we
/// know it's the number from which all numbers after it must be too big).
///
/// Note: `nums` must be sorted when passed to this algorithm.
fn day_1_find_two_numbers_summing_to(nums: &[i32], sum_target: i32) -> Option<i32> {
    let mut b_idx = nums.len() - 1;
    for a in nums.iter() {
        let expected_b = sum_target - a;

        let mut b = nums[b_idx];
        while b >= expected_b {
            if b == expected_b {
                return Some(a * b);
            }

            if b_idx == 0 {
                return None;
            }

            b_idx -= 1;
            b = nums[b_idx];
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{day_1_part_1, day_1_part_2};

    #[test]
    fn day_1_part_1_examples() {
        assert_eq!(day_1_part_1("1721\n979\n366\n299\n675\n1456", 2020), Some(514579));
    }

    #[test]
    fn day_1_part_1_test_input() {
        assert_eq!(day_1_part_1(include_str!("input"), 2020), Some(468051));
    }

    #[test]
    fn day_1_part_2_examples() {
        assert_eq!(day_1_part_2("1721\n979\n366\n299\n675\n1456", 2020), Some(241861950));
    }

    #[test]
    fn day_1_part_2_test_input() {
        assert_eq!(day_1_part_2(include_str!("input"), 2020), Some(272611658));
    }
}
