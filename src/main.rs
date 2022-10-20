use std::{io, ops::Index};
use std::io::Read;

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

fn main() {
    bai_1();
}
