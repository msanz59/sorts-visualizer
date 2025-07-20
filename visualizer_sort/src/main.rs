mod sort;
use std::{io};

fn main() {
    println!("Welcome to the Sorting Visualizer!");
    let mut vec: Vec<i32>;

    loop {
        sort::printer::clear_screen();
        println!("------------------------------------------");
        println!("| Enter a vector of integers:             |");
        println!("| (e.g., 5 3 8 6 2)                       |");
        println!("| Press Enter to exit                     |");
        println!("------------------------------------------");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        vec = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if input.is_empty() {
            break;
        }
        sort::printer::clear_screen();

        println!("------------------------------------------");
        println!("| Chosed vector:                         |");
        // Print the vector left-aligned and padded so the | are aligned
        let vec_str = format!("{:?}", vec);
        let total_width = 40; // width between the | symbols
        let padded_vec = format!("{:<width$}", vec_str, width = total_width);
        println!("|{}|", padded_vec);
        println!("------------------------------------------");
        println!("| Choose a sorting algorithm:            |");
        println!("| 1. Bubble sort                         |");
        println!("| 2. Quick sort                          |");
        println!("| 3. Selection sort                      |");
        println!("| 4. Insertion sort                      |");
        println!("| 5. Merge sort                          |");
        println!("| 6. Exit                                |");
        println!("|                                        |");
        println!("------------------------------------------");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                println!("=== BUBBLE SORT ===");
                sort::bubble::bubble_sort(&mut vec);
                println!("Sorted vector: {:?}", vec);
            }
            "2" => {
                println!("=== QUICK SORT ===");
                if vec.len() > 0 {
                    let len = vec.len();
                    sort::quick::quick_sort(&mut vec, 0, len - 1);
                }
                println!("Sorted vector: {:?}", vec);
            }
            "3" => {
                println!("=== SELECTION SORT ===");
                sort::selection::selection_sort(&mut vec);
                println!("Sorted vector: {:?}", vec);
            }
            "4" => {
                println!("=== INSERTION SORT ===");
                sort::insertion::insertion_sort(&mut vec);
                println!("Sorted vector: {:?}", vec);
            }
            "5" => {
                println!("=== MERGE SORT ===");
                sort::merge::merge_sort(&mut vec);
                println!("Sorted vector: {:?}", vec);

            }
            "6" => break,
            _ => println!("Invalid choice, please try again."),
        }
        println!("Press Enter to continue...");
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).unwrap();
        

        

    }
    
}