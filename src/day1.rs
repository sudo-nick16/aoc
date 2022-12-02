use std::fs;

pub fn main() {

    let content = fs::read_to_string("src/day1.txt").expect("Cound not read file");
    let elf_lists = content.split("\n\n");
    let mut cals:Vec<i32> = vec![];
    for x in elf_lists {
        let mut total = 0;
        for y in x.lines() {
            let ci = y.parse::<i32>();
            if ci.is_ok() {
                total += ci.unwrap();
            }
        }
        cals.push(total);
    }
    cals.sort_by(|a, b| b.cmp(a));
    println!("Answer 1: {}", cals[0]);
    println!("Answer 2: {:?}", &cals[0..3].iter().sum::<i32>());
}
