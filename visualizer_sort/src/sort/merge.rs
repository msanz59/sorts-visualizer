use std::thread;
use std::time::Duration;
use super::printer;

pub fn merge_sort(vec: &mut [i32]) {
    printer::print(vec); // Print the initial state
    let len = vec.len();
    if len <= 1 {
        return;
    }
    merge_sort_helper(vec, 0, len - 1);
}

fn merge_sort_helper(vec: &mut [i32], left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        
        merge_sort_helper(vec, left, mid);
        merge_sort_helper(vec, mid + 1, right);
        
        merge(vec, left, mid, right);
        
        thread::sleep(Duration::from_millis(800));
        printer::clear_screen();
        println!("Merged range [{}, {}]", left, right);
        printer::print(vec);
    }
}

fn merge(vec: &mut [i32], left: usize, mid: usize, right: usize) {
    let left_vec: Vec<i32> = vec[left..=mid].to_vec();
    let right_vec: Vec<i32> = vec[mid + 1..=right].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    
    while i < left_vec.len() && j < right_vec.len() {
        if left_vec[i] <= right_vec[j] {
            vec[k] = left_vec[i];
            i += 1;
        } else {
            vec[k] = right_vec[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left_vec.len() {
        vec[k] = left_vec[i];
        i += 1;
        k += 1;
    }
    
    while j < right_vec.len() {
        vec[k] = right_vec[j];
        j += 1;
        k += 1;
    }
}