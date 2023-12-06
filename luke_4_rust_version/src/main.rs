use std::fs;

fn main() {
    let data: String = fs::read_to_string("reps.txt").unwrap();
    let reps: std::str::Split<'_, &str> = data.split(", ");

    let mut prev_reps: i32 = 0;
    let mut max_streak: i32 = 0;
    let mut max_sum: i32 = 0;
    let mut streak: i32 = 0;
    let mut sum: i32 = 0;

    for i in reps {
        let curr_rep: i32 = i.parse::<i32>().unwrap();
        if curr_rep > prev_reps {
            sum += curr_rep;
            streak += 1;
        } else {
            sum = curr_rep;
            streak = 0;
        }
        if streak > max_streak {
            max_streak = streak;
            max_sum = sum;
        }
        prev_reps = curr_rep;
    }
    println!("Nissen reppet, i den lengste streaken med økende antall reps hver dag, {:?} reps", max_sum);
    print!(" over {:?} dager!", max_streak);
}