use std::thread;
use std::time::Duration;
use super::printer;


pub fn bubble_sort(vec: &mut [i32]) {
    printer::print(vec); // Print the initial state
    
    let n = vec.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                thread::sleep(Duration::from_secs(1));
                printer::clear_screen(); // Clear the console
                printer::print(vec); // Print the state after each swap
                
            }
        }
    }
}