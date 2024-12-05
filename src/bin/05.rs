use std::{collections::HashMap, io};

pub fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut in_rules = true;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let s = line.trim();
        if s.is_empty() {
            in_rules = false;
            continue;
        }

        if in_rules {
            let mut pieces = s.split('|');
            let prev = pieces.next().unwrap().parse().unwrap();
            rule_map
                .entry(pieces.next().unwrap().parse().unwrap())
                .or_default()
                .push(prev);
        } else {
            let mut pages: Vec<u32> = line.split(',').map(|p| p.parse::<u32>().unwrap()).collect();
            let middle_index = pages.len() / 2;
            let mut pass = 0;
            let mut invalid_pages: Vec<u32> = Vec::new();

            while pass == 0 || !invalid_pages.is_empty() {
                invalid_pages.clear();
                let page_order_indexes: HashMap<u32, usize> = pages
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, p)| (p, i))
                    .collect();

                'outer: for (page_index, page) in pages.iter().enumerate() {
                    if let Some(prev_pages) = rule_map.get(page) {
                        for prev_page in prev_pages {
                            if let Some(prev_page_index) = page_order_indexes.get(prev_page) {
                                if *prev_page_index >= page_index {
                                    invalid_pages.push(*page);
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }

                if let Some(invalid_page) = invalid_pages.last() {
                    let existing_index = page_order_indexes.get(&invalid_page).unwrap();

                    let prev_pages = rule_map.get(&invalid_page).unwrap();
                    let min_page = prev_pages
                        .iter()
                        .filter_map(|p| page_order_indexes.get(&p))
                        .max()
                        .unwrap();

                    let page = pages.remove(*existing_index);
                    pages.insert(*min_page, page);
                } else {
                    let middle_page = page_order_indexes
                        .iter()
                        .find(|(_, i)| **i == middle_index)
                        .map(|(p, _)| p)
                        .unwrap();

                    if pass == 0 {
                        part1 += *middle_page;
                    } else {
                        part2 += *middle_page;
                    }
                }

                pass += 1;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
