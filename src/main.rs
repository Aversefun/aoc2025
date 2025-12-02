//! Rust Project

use crate::solutions::SOLUTIONS;

mod solutions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    assert!(args.len() == 2 || args.len() == 3);

    let test_mode = args.len() == 3;

    let (day, part) = args[1].strip_prefix("d").unwrap().split_once("p").unwrap();
    let (day, part): (u8, u8) = (day.parse().unwrap(), part.parse().unwrap());

    let file = std::fs::read_to_string(format!(
        "d{day}-input{}.txt",
        if test_mode {
            format!("-p{part}test")
        } else {
            String::new()
        }
    ))?;

    println!(
        "{}",
        SOLUTIONS[day as usize - 1][part as usize - 1](&file, test_mode)?
    );

    Ok(())
}
