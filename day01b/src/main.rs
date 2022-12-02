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

    let top_elfs = &elf_totals[0..=2];
    let total_calories: u32 = top_elfs.iter().sum();

    println!("The top 3 elfs are carrying {} calories", total_calories);
}