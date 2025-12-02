//! Solutions to the puzzles.

pub const SOLUTIONS: [[fn(&str, bool) -> Result<String, Box<dyn std::error::Error>>; 2]; 2] =
    [[day1p1, day1p2], [day2p1, day2p2]];

fn day1p1(text: &str, test_mode: bool) -> Result<String, Box<dyn std::error::Error>> {
    let input = text.lines();
    let mut state = 50u32;
    let mut password = 0u32;

    for movement in input {
        assert!(movement.len() >= 2);

        let num: u32 = movement[1..].parse()?;
        if num == 0 {
            continue;
        }

        let dir_left = match movement.chars().next().unwrap() {
            'L' => true,
            'R' => false,
            _ => unreachable!(),
        };

        let num: i32 = if dir_left { -(num as i32) } else { num as i32 };

        if let Some(out) = state.checked_add_signed(num) {
            if out <= 99 {
                state = out;
            } else {
                state = out - 100;
                while state > 99 {
                    state -= 100;
                }
            }
        } else {
            let mut temp = 100 + state;
            while temp.checked_add_signed(num).is_none() {
                temp += 100;
            }
            state = temp.strict_add_signed(num);
        }

        if state == 0 {
            password += 1;
        }

        if test_mode {
            eprintln!("{movement}: state: {state}, current password: {password}")
        }
    }
    Ok(format!("password: {password}"))
}

fn day1p2(text: &str, test_mode: bool) -> Result<String, Box<dyn std::error::Error>> {
    let input = text.lines();
    let mut state = 50u32;
    let mut password = 0u32;

    for movement in input {
        assert!(movement.len() >= 2);

        let num: u32 = movement[1..].parse()?;
        if num == 0 {
            continue;
        }

        let dir_left = match movement.chars().next().unwrap() {
            'L' => true,
            'R' => false,
            _ => unreachable!(),
        };

        for _ in 0..num {
            if dir_left {
                if state == 0 {
                    state = 99;
                } else {
                    state -= 1;
                }
            } else {
                state += 1;
                if state == 100 {
                    state = 0;
                }
            }
            if state == 0 {
                password += 1;
            }
        }

        if test_mode {
            eprintln!("{movement}: state: {state}, current password: {password}")
        }
    }
    Ok(format!("password: {password}"))
}

fn day2p1(text: &str, _test_mode: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = 0u64;
    for range in text.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse()?, end.parse()?);

        for i in start..=end {
            let s = i.to_string();
            let split = s.split_at(s.len() / 2);
            if split.0 == split.1 {
                output += i;
            }
        }
    }

    Ok(format!("All added up IDs: {output}"))
}

fn day2p2(text: &str, test_mode: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = 0u64;
    for range in text.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse()?, end.parse()?);

        for i in start..=end {
            let s = i.to_string();
            for repts in 2..=s.len() {
                if s.len() % repts > 0 {
                    if test_mode {
                        eprintln!("no match on repetition {repts} for number {i}: does not divide")
                    }
                    continue;
                }
                let mut splits = Vec::new();
                for r in 0..repts {
                    splits.push(&s[((s.len() / repts) * r)..((s.len() / repts) * (r + 1))]);
                }

                if splits.iter().all(|v| *v == splits[0]) {
                    if test_mode {
                        eprintln!("match on repetition {repts} for number {i}")
                    }
                    output += i;
                    break;
                } else {
                    if test_mode {
                        eprintln!("no match on repetition {repts} for number {i}: not equal")
                    }
                }
            }
        }
    }

    Ok(format!("All added up IDs: {output}"))
}
