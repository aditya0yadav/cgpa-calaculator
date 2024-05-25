use std::io;

#[derive(Debug)] // Add this derive attribute
struct Student {
    name: String,
    maths: f32,
    physics: f32,
    chemistry: f32,
}

fn main() {
    let mut students = Vec::new();

    loop {
        println!("Enter student details or type 'quit' to exit:");

        let mut name = String::new();
        println!("Name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        // Check if the user wants to quit
        let name = name.trim();
        if name.to_lowercase() == "quit" {
            break;
        }

        let mut maths = String::new();
        println!("Maths:");
        io::stdin().read_line(&mut maths).expect("Failed to read line");
        let maths: f32 = maths.trim().parse().expect("Please enter a number");

        let mut physics = String::new();
        println!("Physics:");
        io::stdin().read_line(&mut physics).expect("Failed to read line");
        let physics: f32 = physics.trim().parse().expect("Please enter a number");

        let mut chemistry = String::new();
        println!("Chemistry:");
        io::stdin().read_line(&mut chemistry).expect("Failed to read line");
        let chemistry: f32 = chemistry.trim().parse().expect("Please enter a number");

        let student = Student {
            name: name.to_string(),
            maths,
            physics,
            chemistry,
        };

        students.push(student);
    }

    // Print all student details
    println!("Student details:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: ", i + 1);
        println!("Name: {}", student.name);
        println!("Maths: {}", student.maths);
        println!("Physics: {}", student.physics);
        println!("Chemistry: {}", student.chemistry);
        println!();
    }

    println!("{:?}", students);
}







