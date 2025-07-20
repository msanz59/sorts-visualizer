mod sort;
use std::io;

fn main() {
    // Bubble Sort
    let mut vec1 = vec![5, 3, 8, 6, 2];
    println!("=== BUBBLE SORT ===");
    println!("Original vector: {:?}", vec1);
    sort::printer::print(&vec1);
    sort::bubble::bubble_sort(&mut vec1);
    println!("Sorted vector: {:?}", vec1);
    
    // Wait before next sort
    std::thread::sleep(std::time::Duration::from_secs(2));
    sort::printer::clear_screen();
    
    // Quick Sort with a new unsorted vector
    let mut vec2 = vec![9, 1, 7, 4, 3, 6];
    println!("=== QUICK SORT ===");
    println!("Original vector: {:?}", vec2);
    if vec2.len() > 0 {
        let len = vec2.len();
        sort::quick::quick_sort(&mut vec2, 0, len - 1);
    }
    println!("Sorted vector: {:?}", vec2);

    // Wait before next sort
    std::thread::sleep(std::time::Duration::from_secs(2));
    sort::printer::clear_screen();

    // Selection Sort with a new unsorted vector
    let mut vec3 = vec![4, 2, 5, 1, 3, 6, 8, 7, 9, 0, 10];
    println!("=== SELECTION SORT ===");
    println!("Original vector: {:?}", vec3);
    sort::selection::selection_sort(&mut vec3);
    println!("Sorted vector: {:?}", vec3);

    // Wait before next sort
    std::thread::sleep(std::time::Duration::from_secs(2));
    sort::printer::clear_screen();

    // Insertion Sort with a new unsorted vector
    let mut vec4 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    println!("=== INSERTION SORT ===");
    println!("Original vector: {:?}", vec4);
    sort::insertion::insertion_sort(&mut vec4);
    println!("Sorted vector: {:?}", vec4);

    // Wait before next sort
    std::thread::sleep(std::time::Duration::from_secs(2));
    sort::printer::clear_screen();  

    // Merge Sort with a new unsorted vector
    let mut vec5 = vec![12, 11, 13, 5, 6, 7, 14, 10];
    println!("=== MERGE SORT ===");
    println!("Original vector: {:?}", vec5);
    sort::merge::merge_sort(&mut vec5);
    println!("Sorted vector: {:?}", vec5);
    
    
}