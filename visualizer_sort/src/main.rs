mod sort;

fn main() {
    let mut vec = vec![5, 3, 8, 6, 2];
    println!("Original vector: {:?}", vec);
    sort::printer::print(&vec);
    sort::bubble::bubble_sort(&mut vec);
    println!("Sorted vector: {:?}", vec);
}
