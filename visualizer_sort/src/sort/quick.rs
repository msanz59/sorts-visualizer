use std::thread;
use std::time::Duration;
use super::printer;

pub fn partition(vec: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = vec[high];
    let mut i = low;

    for j in low..high {
        if vec[j] <= pivot {
            if i != j {
                vec.swap(i, j);
                thread::sleep(Duration::from_millis(300));
                printer::clear_screen();
                println!("Swapping {} and {} (pivot: {})", vec[i], vec[j], pivot);
                printer::print(vec);
            }
            i += 1;
        }
    }
    vec.swap(i, high);
    thread::sleep(Duration::from_millis(500));
    printer::clear_screen();
    println!("Moving pivot {} to position {}", pivot, i);
    printer::print(vec);
    
    i  // devolvemos i (que es usize)
}

pub fn quick_sort(vec: &mut [i32], low: usize, high: usize) {
    if low < high {
        
        thread::sleep(Duration::from_millis(500));
        printer::clear_screen();
        printer::print(vec); // Print the state before partitioning
        
        let pi = partition(vec, low, high);

        if pi > 0 {
            quick_sort(vec, low, pi - 1);
        }
        quick_sort(vec, pi + 1, high);
    }
}
