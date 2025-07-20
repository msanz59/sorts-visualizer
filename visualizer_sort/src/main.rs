mod sort;

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

}
