use std::env;

const ALPHA_NUM: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'Z', 'Y', 'Z',
];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return;
    }

    let column: u32 = args[1].parse().unwrap();
    println!("The column disit is {}", to_column(column));
}

fn to_column(num: u32) -> String {
    if num == 0 {
        return String::new();
    }

    let alpha_len = ALPHA_NUM.len() as u32;
    let disit_count = (num - 1) / alpha_len;
    let remain = (num - 1) % alpha_len;

    let top_columns = to_column(disit_count);
    [top_columns, ALPHA_NUM[remain as usize].to_string()].join("")
}
