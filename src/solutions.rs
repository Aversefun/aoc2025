//! Solutions to the puzzles.

pub const SOLUTIONS: [[fn(core::str::Lines<'_>, bool) -> Result<String, Box<dyn std::error::Error>>; 2]; 1] = [[day1p1, day1p2]];

fn day1p1(input: core::str::Lines<'_>, test_mode: bool) -> Result<String, Box<dyn std::error::Error>> {
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
            _ => unreachable!()
        };

        let num: i32 = if dir_left {
            -(num as i32)
        } else {
            num as i32
        };
        

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

fn day1p2(input: core::str::Lines<'_>, test_mode: bool) -> Result<String, Box<dyn std::error::Error>> {
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
            _ => unreachable!()
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
