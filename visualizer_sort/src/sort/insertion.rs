use std::thread;
use std::time::Duration;
use super::printer;

pub fn insertion_sort(vec: &mut [i32]) {
    let len = vec.len();
    printer::print(vec); // Print the initial state
    
    for i in 1..len {
        let mut j = i;
        while j > 0 && vec[j] < vec[j-1] {
            vec.swap(j, j-1);
            j -= 1;
            thread::sleep(Duration::from_secs(1));
            printer::clear_screen(); // Clear the console
            printer::print(vec); // Print the state after each swap
        }
    }
}