use std::collections::HashMap;
use std::io;

fn main() {
    println!("Mean, median, and mode calculator");
    println!("Enter list of sorted comma-separated numbers");

    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Failed read line");

    let buf = buf.replace(" ", "");
    let str_nums = buf.trim().split(",");
    let mut nums = Vec::new();
    let mut num_freq = HashMap::new();
    let mut total = 0.0;
    for str_num in str_nums {
        let num: f64 = match str_num.parse() {
            Ok(num) => num,
            Err(e) => panic!("{}", e),
        };

        let freq = num_freq.entry(str_num).or_insert(0);
        *freq += 1;
        total += num;
        nums.push(num);
    }

    let nums_len = nums.len();
    let middle = (nums_len + 1) / 2 - 1;
    let med = if (nums_len + 1) % 2 == 0 {
        f64::from(nums[middle])
    } else {
        let upper_num = f64::from(nums[middle + 1]);
        let lower_num = f64::from(nums[middle]);
        (upper_num + lower_num) / 2.0
    };

    let mut max: Option<i32> = None;
    // Find highest number frequency.
    for (_, freq) in &num_freq {
        max = match max {
            Some(old_freq) => {
                if old_freq < *freq {
                    Some(freq);
                }

                Some(old_freq)
            }
            None => Some(*freq),
        };
    }

    let mut str_mode = Vec::new();

    for (str_num, freq) in &num_freq {
        if *freq == max.unwrap() {
            str_mode.push(*str_num)
        }
    }

    str_mode.sort();

    let mut num_mode = Vec::new();
    for mode in str_mode {
        let f64_mode: f64 = mode.parse().unwrap();
        num_mode.push(f64_mode)
    }

    println!("Given {:?}", nums);
    println!("Mean is {}", total / nums.len() as f64);
    println!("Median is {}", med);
    println!("Mode is/are {:?}", num_mode)
}
