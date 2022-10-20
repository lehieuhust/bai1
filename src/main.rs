use std::{io, ops::Index};
use std::io::Read;
use std::str::Split;

fn bai_1() {
    println!("bai 1");
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let org_len = org_arr.len();
    let sub_len = sub_arr.len();
    let mut i = 0;
    let mut j = 0;

    while i < org_len && j < sub_len {
        if org_arr[i] == sub_arr[j] {
            println!("org_arr[{}] = {}",i, org_arr[i]);
            println!("sub_arr[{}] = {}",j, sub_arr[j]);
            i+=1;
            j+=1;

            if j == sub_len {
                println!("PASS");
            }
        }
        else {
            i = i - j + 1;
            j = 0;
        }
    }
}

fn bai_2() {
    let input = "adbcdaDd".to_lowercase();
    let mut input_ch = String::new();
    std::io::stdin().read_line(&mut input_ch);
    let input_ch = input_ch.to_lowercase().chars().next().unwrap();
    println!("input_ch: {}", input_ch);

    let mut str: String = (*input).to_string();
    let mut cnt = 0;
    for (i, ch) in input.chars().enumerate() {
        if input_ch == ch {
            str.remove(i - cnt);
            cnt += 1;
        }
    }
    println!("str: {}", str);
}

fn main() {
    bai_1();
    bai_2();
}
