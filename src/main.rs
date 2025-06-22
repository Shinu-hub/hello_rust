
      use std::io;

fn main() {
    let mut name = String::new();
    let mut marks = String::new();
    let mut subs = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut marks).unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subs).unwrap();

    let marks: f32 = marks.trim().parse().unwrap();
    let subs: f32 = subs.trim().parse().unwrap();
    let avg = marks / subs;

    let grade = if avg >= 90.0 {
        'A'
    } else if avg >= 75.0 {
        'B'
    } else if avg >= 60.0 {
        'C'
    } else {
        'D'
    };

    println!("\n--- Report ---");
    println!("Name: {}", name.trim());
    println!("Average: {:.2}", avg);
    println!("Grade: {}", grade);
}

