use crate::utils::load_input;
const DAY: u32 = 5;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut rules: Vec<(i32,i32)> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    // get input into the rules and updates variables
    for line in lines {
        if let Some(iter) = line.split_once('|') {
            rules.push((
                iter.0.parse().unwrap(),
                iter.1.parse().unwrap(),
            ));
        } else if line.contains(',') {
            updates.push(Update {
                list: line
                    .split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            });
        }
    }
    let result_part1 = part1(&rules, &updates);
    let result_part2 = part2(&rules, &updates);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Debug)]
struct Update {
    list: Vec<i32>,
}

fn part1(rules: &[(i32,i32)], updates: &Vec<Update>) -> i32 {
    let mut result = 0;

    for update in updates {
        result += check_update(rules, update);
    }

    result
}

// return value of middle page of update, if rules check out, else return 0
fn check_update (rules: &[(i32,i32)], update: &Update) -> i32 {
    let mut past_pages: Vec<i32> = Vec::new();

    // go through all numbers in update
    for page in &update.list {
        // collect all rules that need to be checked
        let checks: Vec<(i32, i32)> = rules.iter().filter(|&&(first, _)| first == *page).copied().collect();
        // if a rule does not check out, return 0
        for check in checks {
            if past_pages.iter().any(|x| *x == check.1) {return 0}
        }
        past_pages.push(*page);
    }

    // return the middle page
    update.list[update.list.len().div_ceil(2)-1]
}

fn part2(rules: &Vec<(i32,i32)>, updates: &Vec<Update>)  -> i32 {
    let mut result = 0;

    for update in updates {
        result += correct_update(rules, update, false);
    }

    result
}

// return value of middle page of a previously faulty update, if update was alright return 0
fn correct_update (rules: &Vec<(i32,i32)>, update: &Update, has_been_changed: bool) -> i32 {
    let mut past_pages: Vec<i32> = Vec::new();

    // go through all numbers in update
    for page in &update.list {
        // collect all rules that need to be checked
        let checks: Vec<(i32, i32)> = rules.iter().filter(|&&(first, _)| first == *page).copied().collect();
        // if a rule does not check out, swap the correlating numbers and run correct_update on the new  order
        for check in checks {
            if past_pages.iter().any(|x| *x == check.1) {
                // clone of update but swap the numbers of the check that failed
                let new_update = Update{list: update.list.iter().copied()
                    .map(|x| match x {
                        x if x == check.0 => check.1,
                        x if x == check.1 => check.0,
                        _ => x,
                    })
                    .collect()};
                return correct_update(rules, &new_update, true)
            }
        }
        past_pages.push(*page);
    }

    // return the middle page or 0, if nothing was changed.
    if has_been_changed {update.list[update.list.len().div_ceil(2)-1]} else {0}
}