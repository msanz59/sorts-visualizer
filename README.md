# Sorting Algorithms Visualizer

A command-line sorting algorithms visualizer built in Rust that provides step-by-step visual representation of how different sorting algorithms work.

## Features

- **Interactive CLI Interface**: Choose from multiple sorting algorithms through a user-friendly menu
- **Real-time Visualization**: Watch each step of the sorting process with ASCII art representation
- **Multiple Algorithms**: Implementation of 5 classic sorting algorithms
- **Step-by-step Animation**: Each swap/operation is displayed with timing delays for better understanding

## Implemented Algorithms

### 1. Bubble Sort
- **Time Complexity**: O(n²) average and worst case, O(n) best case
- **Space Complexity**: O(1)
- **Description**: Repeatedly steps through the list, compares adjacent elements and swaps them if they're in the wrong order. The pass through the list is repeated until the list is sorted.

### 2. Quick Sort
- **Time Complexity**: O(n log n) average case, O(n²) worst case
- **Space Complexity**: O(log n)
- **Description**: Uses a divide-and-conquer approach by selecting a 'pivot' element and partitioning the array around it, then recursively sorting the sub-arrays.

### 3. Selection Sort
- **Time Complexity**: O(n²) in all cases
- **Space Complexity**: O(1)
- **Description**: Finds the minimum element in the unsorted portion and swaps it with the first unsorted element, gradually building the sorted portion from left to right.

### 4. Insertion Sort
- **Time Complexity**: O(n²) average and worst case, O(n) best case
- **Space Complexity**: O(1)
- **Description**: Builds the final sorted array one item at a time by repeatedly taking an element from the unsorted portion and inserting it into its correct position in the sorted portion.

### 5. Merge Sort
- **Time Complexity**: O(n log n) in all cases
- **Space Complexity**: O(n)
- **Description**: Uses divide-and-conquer by recursively dividing the array into halves until single elements remain, then merges them back together in sorted order.

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Command-line terminal

## Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/sorts-visualizer.git
   cd sorts-visualizer/visualizer_sort
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the program:**
   ```bash
   cargo run
   ```

## Usage

1. **Launch the program** using `cargo run`

2. **Enter a vector of integers** when prompted (e.g., `5 3 8 6 2`)
   - Use space-separated integers
   - Press Enter with empty input to exit

3. **Choose a sorting algorithm** from the menu:
   - Press `1` for Bubble Sort
   - Press `2` for Quick Sort
   - Press `3` for Selection Sort
   - Press `4` for Insertion Sort
   - Press `5` for Merge Sort
   - Press `6` to Exit

4. **Watch the visualization** as the algorithm sorts your data step by step

5. **Press Enter** after completion to return to the main menu

## Example Output

```
Welcome to the Sorting Visualizer!
------------------------------------------
| Enter a vector of integers:             |
| (e.g., 5 3 8 6 2)                       |
| Press Enter to exit                     |
------------------------------------------
5 3 8 6 2

------------------------------------------
| Chosed vector:                         |
| [5, 3, 8, 6, 2]                        |
------------------------------------------
| Choose a sorting algorithm:            |
| 1. Bubble sort                         |
| 2. Quick sort                          |
| 3. Selection sort                      |
| 4. Insertion sort                      |
| 5. Merge sort                          |
| 6. Exit                                |
------------------------------------------

=== BUBBLE SORT ===
8: 	    *     
7: 	    *     
6: 	    * *   
5: 	*   * *   
4: 	*   * *   
3: 	* * * *   
2: 	* * * * * 
1: 	* * * * * 
   ========== 
```

## Project Structure

```
visualizer_sort/
├── src/
│   ├── main.rs              # Main program entry point
│   └── sort/
│       ├── mod.rs           # Module declarations
│       ├── bubble.rs        # Bubble sort implementation
│       ├── quick.rs         # Quick sort implementation
│       ├── selection.rs     # Selection sort implementation
│       ├── insertion.rs     # Insertion sort implementation
│       ├── merge.rs         # Merge sort implementation
│       └── printer.rs       # Visualization utilities
├── Cargo.toml               # Project dependencies
└── README.md               # This file
```

## Performance Notes

- **Timing**: Each algorithm includes sleep delays for visualization purposes
- **Input Size**: For better visualization, recommended input size is 2-15 integers
- **Values Range**: Works best with positive integers 1-20 for clear ASCII representation

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Author

Built with ❤️ using Rust
