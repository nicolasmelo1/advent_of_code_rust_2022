use std::{fs};

pub fn run() {
    let contents = fs::read_to_string("./src/day1/input.txt")
        .expect("Should have been able to read the file");

    let mut splitted: Vec<&str> = contents.split('\n').collect();
    // this will guarantee that we go to the last element
    splitted.push("");
    let mut top_three: Vec<i32> = Vec::new();
    let mut sum = 0;

    for text in &splitted {
        match &text[..] {
            "" => {
                let top_three_is_not_full = top_three.len() < 3;
                if top_three_is_not_full {
                    top_three.push(sum)
                } else {
                    for index in 0..top_three.len() {
                        let top = top_three.get(index).unwrap();
                        if sum > *top {
                            top_three.splice(index..index+1, [sum]);
                            top_three.sort();
                            break;
                        }
                    }
                }
                sum = 0;
            },
            _ => {
                let to_number = text.parse::<i32>().unwrap();
                sum = sum + to_number;
            }
        }
    }
    println!("{:#?}", top_three);
    println!("{:#?}", top_three.into_iter().reduce(|accumulated, item| accumulated + item).unwrap());

}
