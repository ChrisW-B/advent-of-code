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
    // x, y
    let mut loc: VecDeque<i32> = VecDeque::from([0, 0]);

    let mut next_ln = read_line_buffer();

    while next_ln != "\n" {
        let dir_dist = next_ln.split_whitespace().collect::<Vec<&str>>();
        let dir = dir_dist[0];
        let dist: i32 = dir_dist[1].trim().parse().unwrap();

        if dir == "forward" {
            loc[0] = loc[0] + dist;
        } else if dir == "up" {
            loc[1] = loc[1] - dist;
        } else if dir == "down" {
            loc[1] = loc[1] + dist;
        }

        next_ln = read_line_buffer();
    }

    print!("x: {}, y: {}, total: {}\n", loc[0], loc[1], loc[0] * loc[1])
}
