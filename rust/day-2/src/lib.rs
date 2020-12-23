use std::ops::RangeInclusive;

#[allow(dead_code)]
fn day_2_part_1(input: &str) -> usize {
    let passcodes  = day_2_parse(input);
    let validator = CharCountRuleValidator {};
    passcodes.iter().filter(|p| p.is_valid(&validator)).count()
}

#[allow(dead_code)]
fn day_2_part_2(input: &str) -> usize {
    let passcodes  = day_2_parse(input);
    let validator = CharPositionRuleValidator {};
    passcodes.iter().filter(|p| p.is_valid(&validator)).count()
}

fn day_2_parse(input: &str) -> Vec<PasscodeAndRule> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line_parts = line.split(": ");
            let rule = line_parts.next().expect(&format!("Line was not split by ': ': {}", line));
            let passcode = line_parts.next().expect(&format!("Line was not split by ': ': {}", line))
                .to_string();

            let mut rule_parts = line.split(' ');
            let range = rule_parts.next().expect(&format!("Rule was not split by ' ': {}", rule));
            let req_char = rule_parts.next().expect(&format!("Rule was not split by ' ': {}", rule))
                .chars().next().expect("Required char was not a single char");

            let mut range_parts = range.split('-');
            let range_start_raw = range_parts.next().expect(&format!("Range was not split by '-': {}", range));
            let range_end_raw = range_parts.next().expect(&format!("Range was not split by '-': {}", range));

            let range_start = range_start_raw.parse::<usize>().expect(&format!("Could not parse range start '{}' as usize.", range_start_raw));
            let range_end = range_end_raw.parse::<usize>().expect(&format!("Could not parse  range end'{}' as usize.", range_end_raw));

            let rule = Rule {
                req_char,
                range: (range_start..=range_end)
            };

            PasscodeAndRule {
                rule,
                passcode
            }
        })
        .collect()
}

struct Rule {
    req_char: char,
    range: RangeInclusive<usize>
}

struct PasscodeAndRule {
    rule: Rule,
    passcode: String
}

impl PasscodeAndRule {
    fn is_valid(&self, validator: &dyn RuleValidator) -> bool {
        validator.is_valid(&self.passcode, &self.rule)
    }
}

trait RuleValidator {
    fn is_valid(&self, passcode: &str, rule: &Rule) -> bool;
}

struct CharCountRuleValidator {}

impl RuleValidator for CharCountRuleValidator {
    fn is_valid(&self, passcode: &str, rule: &Rule) -> bool {
        let req_chars = passcode.chars()
            .filter(|c| c == &rule.req_char)
            .count();

        rule.range.contains(&req_chars)
    }
}

struct CharPositionRuleValidator {}

impl RuleValidator for CharPositionRuleValidator {
    fn is_valid(&self, passcode: &str, rule: &Rule) -> bool {
        let pos_one = passcode.chars().nth(*rule.range.start() - 1);
        let pos_two = passcode.chars().nth(*rule.range.end() - 1);

        let expected = Some(rule.req_char);

        (pos_one == expected && pos_two != expected) ||
            (pos_one != expected && pos_two == expected)
    }
}

#[cfg(test)]
mod tests {
    use super::{day_2_part_1, day_2_part_2};

    #[test]
    fn day_2_part_1_examples() {
        assert_eq!(day_2_part_1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn day_2_part_1_test_input() {
        assert_eq!(day_2_part_1(include_str!("input")), 500);
    }

    #[test]
    fn day_2_part_2_examples() {
        assert_eq!(day_2_part_2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }

    #[test]
    fn day_2_part_2_test_input() {
        assert_eq!(day_2_part_2(include_str!("input")), 313);
    }
}
