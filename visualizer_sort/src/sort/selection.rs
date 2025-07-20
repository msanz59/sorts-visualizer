use std::thread;
use std::time::Duration;
use super::printer;

pub fn selection_sort(vec: &mut [i32]) {
    let n = vec.len();
    printer::print(vec); // Print the initial state

    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if vec[j] < vec[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            vec.swap(i, min_index);
            thread::sleep(Duration::from_secs(1));
            printer::clear_screen(); // Clear the console
            printer::print(vec); // Print the state after each swap
        }
    }
}
