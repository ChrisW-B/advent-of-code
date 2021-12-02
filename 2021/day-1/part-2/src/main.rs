use std::collections::VecDeque;
use std::io;

fn read_line_buffer() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input
}

fn main() {
    let mut total_greater = 0;
    let mut prev_sum = -1;

    let mut window: VecDeque<i32> = VecDeque::new();

    let mut next_ln = read_line_buffer();

    while next_ln != "\n" {
        let next_int: i32 = next_ln.trim().parse().unwrap();

        window.push_front(next_int);
        let new_sum = window.iter().sum();

        if window.len() == 3 && new_sum > prev_sum && prev_sum != -1 {
            total_greater = total_greater + 1;
        }
        if window.len() == 3 {
            prev_sum = new_sum;
            window.pop_back();
        }

        next_ln = read_line_buffer();
    }

    print!("total greater: {}", total_greater)
}
