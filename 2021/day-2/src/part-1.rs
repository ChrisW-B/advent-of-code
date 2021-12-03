use std::io;

fn read_line_buffer() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input
}

fn main() {
    let mut x_loc = 0;
    let mut y_loc = 0;

    let mut next_ln = read_line_buffer();

    while next_ln != "\n" {
        let dir_dist = next_ln.split_whitespace().collect::<Vec<&str>>();
        let dir = dir_dist[0];
        let dist: i32 = dir_dist[1].trim().parse().unwrap();

        if dir == "forward" {
            x_loc = x_loc + dist;
        } else if dir == "up" {
            y_loc = y_loc - dist;
        } else if dir == "down" {
            y_loc = y_loc + dist;
        }

        next_ln = read_line_buffer();
    }

    print!("x: {}, y: {}, total: {}\n", x_loc, y_loc, x_loc * y_loc)
}
