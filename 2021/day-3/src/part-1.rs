use std::io;

fn read_line_buffer() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input
}

fn main() {
    let mut next_ln = read_line_buffer();
    let mut counter = vec![vec![0; 2]; next_ln.trim().len()];

    while next_ln != "\n" {
        let char_vec = next_ln.trim().chars();

        for (i, ch) in char_vec.enumerate() {
            let char_val: usize = (ch.to_string()).parse().unwrap();
            counter[i][char_val] = counter[i][char_val] + 1;
        }

        next_ln = read_line_buffer();
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for pair in counter.iter() {
        if pair[0] < pair[1] {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_val = isize::from_str_radix(gamma.as_ref(), 2).unwrap();
    let epsilon_val = isize::from_str_radix(epsilon.as_ref(), 2).unwrap();

    println!(
        "gamma: {}, epsilon: {}, total: {}",
        gamma_val,
        epsilon_val,
        gamma_val * epsilon_val
    );
}
