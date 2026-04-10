pub fn variables() {
    let negative_number: i32 = -56; 
    println!("Negative number (signed): {}", negative_number);

    let positive_only_number: u32 = 56; 
    println!("Positive only number (unsigned): {}", positive_only_number);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);
    println!("Last fruit: {}", fruits[2]);

    // tuples
    let coordinates: (i32, i32) = (10, 20);
    println!("Coordinates: ({}, {})", coordinates.0, coordinates.1);

    let coordinates2: (i32, i32) = (10, 46);
    println!("Coordinates2: ({}, {})", coordinates2.0, coordinates2.1);

    let car: (&str, i32) = ("Toyota", 2020);
    println!("Car: {} ({})", car.0, car.1);

    let car2: (String, i32) = ("Subaru".to_string(), 2020);
    println!("Car2: {} ({})", car2.0, car2.1);

    tuples();
    slices();

    // Strings VS String Slices (&str)
    // Strings [Growable, Mutable, Owned]

    let final_var: String = String::from("Final value"); // immutable so you can modify later
    println!("Final variable: {}", final_var);

    let mut mutable_var: String = String::from("Mutable value"); // mutable string
    println!("Mutable variable: {}", mutable_var);
    mutable_var.push_str(" - modified");
    println!("Modified mutable variable: {}", mutable_var);

    // B- &str (String Slice)
    let string: String = String::from("Hello, world!");
    let str_slice: &str = &string;
    println!("String slice: {}", str_slice);

    let sliced : &str = &string[0..5];
    println!("Sliced string: {}", sliced);

    expression_and_statements();
}

fn tuples() {
    let coordinates: (i32, i32) = (10, 20);
    println!("Coordinates: ({}, {})", coordinates.0, coordinates.1);

    let coordinates2: (i32, i32) = (10, 46);
    println!("Coordinates2: ({}, {})", coordinates2.0, coordinates2.1);

    let car: (&str, i32) = ("Toyota", 2020);
    println!("Car: {} ({})", car.0, car.1);

    let car2: (String, i32) = ("Subaru".to_string(), 2020);
    println!("Car2: {} ({})", car2.0, car2.1);
}

fn slices() {
     let numbers_slice: &[i32] = &[1,2,3,4,5,6,7];
    println!("Numbers slice: {:?}", numbers_slice);

    let animal_slices: &[&str] = &["cat", "dog", "bird"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"1984".to_string(), &"To Kill a Mockingbird".to_string(), &"The Great Gatsby".to_string()];
    println!("Book slices: {:?}", book_slices);

}


pub fn tell_height(height: u32) {
    println!("The height is: {}", height);
}

pub fn human_info(name: &str, age: u32, height: f32) {
    println!("The human's name is: {}", name);
    println!("The human's age is: {}", age);
    println!("The human's height is: {}", height);
    println!("My name is: {} and I am {} years old.", name, age);
}


fn expression_and_statements() {
    // Expression is anything that returns a value
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x: {}, y: {}", x, y);

    let z : i32 = { // this is something new compared to other programming languages
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty //  this is like return
    };
    println!("z: {}", z);

}