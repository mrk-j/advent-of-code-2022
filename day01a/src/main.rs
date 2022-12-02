use std::fs;

fn main() {
    let mut elf_totals: Vec<u32> = vec![];
    let mut current_elf_total: u32 = 0;

    let _contents = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                elf_totals.push(current_elf_total);
                current_elf_total = 0;
            } else {
                current_elf_total += line.parse::<u32>().unwrap();
            }
        });

    elf_totals.push(current_elf_total);
    elf_totals.sort_by(|a, b| b.cmp(a));

    println!("The elf carrying the most calories is carrying {} calories", elf_totals.get(0).unwrap());
}