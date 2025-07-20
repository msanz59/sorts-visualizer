use std::process::Command;

pub fn print(vec: &[i32]) {
    let lon = vec.len();
    let max = *vec.iter().max().unwrap_or(&0);
    for i in (1..=max).rev() {
        print!("{}: ", i); // Print the current level
        for &value in vec {
            if value >= i {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    print!("  ");
    println!("{}", "==".repeat(lon)); // Print a line of underscores to represent the base
    
}

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        // For Unix-like systems (Linux, macOS)
        Command::new("clear").status().unwrap();
    }
}