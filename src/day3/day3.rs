use std::{ fs, str::Split, collections::{ HashMap, HashSet } };

fn get_score_table() -> HashMap<char, i32> {
    return HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);
}

fn split_by_half(input: &str) -> (String, String) {
    let half = input.len() / 2;
    let (left, right) = input.split_at(half);
    (left.to_string(), right.to_string())
}

fn iterate_over_to_find_equal_items(compartment1: String, compartment2: String) -> HashSet<char> {
    let mut equal_items = HashSet::new();
    for item in compartment1.chars() {
        if compartment2.contains(item) {
            equal_items.insert(item);
        }
    }
    return equal_items;
}

fn get_score_for_equal_items(equal_items: HashSet<char>) -> i32 {
    let score_table = get_score_table();
    let mut score = 0;
    for item in equal_items {
        score = score + score_table.get(&item).unwrap();
    }
    return score;
}

fn part1(splitted_contents: Split<char>) {
    let mut sum_of_scores = 0;
    for rucksack in splitted_contents {
        let (left, right) = split_by_half(rucksack);
        let equal_items = iterate_over_to_find_equal_items(left, right);
        let score = get_score_for_equal_items(equal_items);
        sum_of_scores = sum_of_scores + score;
    }
    println!("Part 1: {}", sum_of_scores);
}


pub fn run() {
    let contents = fs::read_to_string("./src/day3/input.txt")
        .expect("Should have been able to read the file");

    let splitted = contents.split('\n');
    part1(splitted);
}