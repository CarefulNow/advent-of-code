use appendlist::AppendList;
use std::fs;

fn main() {
    let calories = fs::read_to_string("./day-1/input.txt")
        .expect("Unable to read file");


    let elves: AppendList<u32> = AppendList::new();
    let mut elf_calorie_count: u32 = 0;
    for line in calories.lines() {
        if line != "" {
            elf_calorie_count +=line.parse::<u32>().unwrap();
        } else {
            elves.push(elf_calorie_count);
            elf_calorie_count = 0;
        }
    }

    let mut elf_with_max_calories: [u32; 3] = [0, 0, 0];
    for elf in &elves {
        elf_with_max_calories.sort();
        for fat_elf in elf_with_max_calories.iter_mut() {
            if elf > fat_elf {
                *fat_elf = *elf;
                break;
            }
        }
    }

    let elves_with_max_calorie_sum : u32 = elf_with_max_calories.iter().sum();
    println!("Total: {}", elves_with_max_calorie_sum);
}
