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
    let mut prev_num = String::new();
    let mut new_num = read_line_buffer();

    while new_num != "\n" {
        if  new_num > prev_num {
            total_greater = total_greater + 1;
        }
        prev_num = new_num;
        new_num = read_line_buffer();
    }

    print!("total greater: {}", total_greater)
}
