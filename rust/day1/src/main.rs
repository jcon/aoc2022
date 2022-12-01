use std::str::Lines;

struct Cli {
    pub path: std::path::PathBuf,
}

// NOTE: could use something like clap instead.
impl Cli {
    pub fn parse() -> Self {
        let path = std::env::args().nth(1).expect("no path given");
        Cli {
            path: std::path::PathBuf::from(path),
        }
    }
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    max_calories(content.lines());
    max_3_calories(content.lines());
}

fn max_calories(lines: Lines<'_>) {
    let mut calories = Vec::new();
    let mut cur = 0;
    for line in lines {
        if line.is_empty() {
            calories.push(cur);
            cur = 0;
            continue;
        }
        let calories = line.parse::<i32>().unwrap();
        cur += calories;
    }
    calories.push(cur);
    calories.sort_by(|a, b| b.cmp(a));
    let max = calories[0];
    println!("elf with max calories: {}", max);
}

// If the number of elves were large, we can reduce the runtime from n*log n => lg n
// by using a BinaryHeap instead of sorting.
fn max_3_calories(lines: Lines<'_>) {
    let mut calories = Vec::new();
    let mut cur = 0;
    for line in lines {
        if line.is_empty() {
            calories.push(cur);
            cur = 0;
            continue;
        }
        let calories = line.parse::<i32>().unwrap();
        cur += calories;
    }
    calories.push(cur);
    calories.sort_by(|a, b| b.cmp(a));
    let max = calories[0];
    println!("elf with max calories: {}", max);
    println!(
        "elves with top3 calories: {} ({} {} {})",
        calories[0] + calories[1] + calories[2],
        calories[0],
        calories[1],
        calories[2]
    );
}
