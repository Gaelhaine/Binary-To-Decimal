use std::env;

fn main() {
    let args = env::args().skip(1).collect::<String>();
    let argc = args.split("").collect::<Vec<&str>>();
    let mut count = 0;
    let mut reset = 0;
    println!("{:?}", argc);
    for number in argc.iter().rev() {
        if reset == 8 {
            reset = 0;
        }
        match *number {
            "0" => {
                count += 0;
                reset += 1;
            }
            "1" => {
                count = (2i32.pow(reset)) + count;
                reset += 1;
            }
            "" => count += 0,
            _ => {
                println!("Mmmmmm, looks like something went wrong, please check your input");
                std::process::exit(1);
            }
        }
    }

    println!("{}", count);
}