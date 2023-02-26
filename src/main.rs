use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elves_cals: Vec<Vec<i32>> = vec![];
        let mut elf_cals: Vec<i32> = vec![];
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                if ip == "" {
                    elves_cals.push(elf_cals.clone());
                    elf_cals.clear();
                    continue;
                }
                match ip.parse::<i32>() {
                    Ok(ip_numeric) => elf_cals.push(ip_numeric),
                    Err(_) => {
                        println!("Error parsing {}", ip);
                    }
                }
            }
        }
        elves_cals.push(elf_cals.clone());
        println!(
            "Total: {:?}",
            elves_cals
                .iter()
                .map(|x| x.iter().sum::<i32>())
                .collect::<Vec<i32>>()
        );

        // Part 1
        if let Some(max) = elves_cals
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .collect::<Vec<i32>>()
            .iter()
            .max()
        {
            println!("Max: {}", max);
        }

        // Part 2
        let mut summed_elves_cals: Vec<i32> = elves_cals
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .collect::<Vec<i32>>();
        summed_elves_cals.sort();
        let top_3_cals: Vec<i32> = summed_elves_cals
            .iter()
            .copied()
            .rev()
            .take(3)
            .collect::<Vec<i32>>();
        println!("{:?}", top_3_cals.iter().sum::<i32>());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
