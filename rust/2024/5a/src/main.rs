use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn sum_page_numbers(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<_>>();
    let mut page_updates: Vec<&str> = Vec::new();
    let mut pages_ordering_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut push_updates = false;
    lines.iter().for_each(|line| {
        let line_value = *line;
        if !push_updates {
            if line_value == "" {
                push_updates = true;
                return;
            }

            let page_number = line_value.split("|").collect::<Vec<_>>();
            let first = page_number.get(0).unwrap();
            let second = page_number.get(1).unwrap();
            if pages_ordering_rules.contains_key(first) {
                pages_ordering_rules.get_mut(first).unwrap().push(second);
            } else {
                pages_ordering_rules.insert(first, vec![second]);
            }

            return;
        }

        page_updates.push(line);
    });

    let correct_orders = page_updates
        .iter()
        .filter(|line| {
            let line_value = *line;
            let line_collection = line_value.split(",").collect::<Vec<_>>();
            let line_size = line_collection.len();
            (0..line_size).all(|i| {
                let current_number = line_collection.get(i).unwrap();
                (i..line_size).all(|j| {
                    let next_number = line_collection.get(j).unwrap();

                    let rules = pages_ordering_rules.get(next_number);
                    if rules.is_none() {
                        return true;
                    }
                    let rules = rules.unwrap();
                    !rules.contains(current_number)
                })
            })
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for order in correct_orders {
        let order_value = *order;
        if order_value == "" {
            continue;
        }
        let order_collection = order_value.split(",").collect::<Vec<_>>();
        let order_size = order_collection.len() - 1;
        let middle_value_index = (order_size as f64 / 2.0).ceil() as i32;
        let middle_value = order_collection.get(middle_value_index as usize).unwrap();
        sum += middle_value.parse::<i32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::sum_page_numbers;

    #[test]
    fn test_simple() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(sum_page_numbers(input), 143);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!(sum_page_numbers(input), 4957);
    }
}
