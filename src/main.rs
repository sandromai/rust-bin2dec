use std::env;

fn bin_to_dec(bin: &String) -> String {
    let mut acc : i32 = 0;

    for ch in bin.chars() {
        acc = acc * 2;

        if ch == '1' {
            acc += 1;
        } else if ch != '0' {
            return format!("Invalid binary '{}'.", ch);
        }
    }

    return format!("{}", acc);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No arguments provided.");

        return;
    }

    println!("{}", bin_to_dec(&args[1]));
}
