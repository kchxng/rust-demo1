use std::collections::HashMap;
// use rust_demo1::Person;
// use rust_demo1::person::Person;
use rust_demo1::{customer::Customer, person::Person, speaking::Speaking};
fn main() {
    println!("Hello, world!");
    let score = 80;
    let result = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    let arr = [2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Your score is: {}", result);
    for i in 0..arr.len() {
        println!("The value of arr[{}] is: {}", i, arr[i]);
    }
    get_num();

    // HashMap
    // let mut scores:HashMap<&str,i32>= std::collections::HashMap::new();
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 85);
    scores.insert("Bob", 90);
    scores.insert("Charlie", 78);

    let y1 = scores.get("Alice").unwrap();
    println!("Alice's score: {}", y1);
    let y2 = scores.get("Bob").unwrap();
    println!("Bob's score: {}", y2);

    for (name, score) in &scores {
        println!("HashMap: {}: {}", name, score);
    }

    // Vector
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    for i in &vec {
        println!("Vec: {}", i);
    }

    // Struct
    let p = Person::new("Cheng".to_string(), 28);
    p.hello();

    let c = Customer::new("Cheng".to_string(), "55559999".to_string());
    c.hello();

    //Trait
    p.speak();

    // Enum
    let x = Colors::Orange;
    // Match is like a Switch case in golang or typescript or Java
    match x {
        Colors::Red => println!("Red"),
        Colors::Green => println!("Green"),
        Colors::Blue => println!("Blue"),
        Colors::Orange => println!("Orange"),
        _ => println!("Other"),
    }

    let xx = check_grade(90);
    match xx {
        GradeResult::Value(v) => println!("Grade is Value :{}", v),
        GradeResult::Error(msg) => println!("Grade is Error: {}", msg),
    }

    let xxx = check_grade2(80);
    match xxx {
        Some(v) => println!("You are {}", v),
        None => println!("You're failed!"),
    }

    let x = check_grade3(100);
    match x {
        Ok(v) => println!("{}", v),
        Err(msg) => println!("{}", msg),
    }

    let x = check_grade3(90);
    let y = match x {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(v) => v,
    };
    println!("Grade is {}", y);

    // Closure
    let x = add(10, 20);
    println!("x is {}", x);
    let x = |a: i32, b: i32| a + b; // Closure like this line
    println!("x is {}", x(50, 40));
    let y = cal(10, 100, x);
    println!("y is {}", y);
    let y = cal(10, 40, add);
    println!("y is {}", y);
    let y = cal(10, 110, |a: i32, b: i32| a + b);
    println!("y is {}", y);
}

fn cal<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f: F) -> i32 {
    f(a, b)
}
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score >= 100 {
        return GradeResult::Error("score is not correct".to_string());
    }
    return GradeResult::Value(score);
}

fn check_grade2(score: i32) -> Option<String> {
    if score < 50 {
        return None;
    }
    return Some("Pass".to_string());
}

fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score >= 100 {
        return Err("score is not correct".to_string());
    }
    return Ok("A".to_string());
}
enum GradeResult {
    Value(i32),
    Error(String),
}

enum Colors {
    Red,
    Green,
    Blue,
    Orange,
}

fn get_num() -> i32 {
    // println!("Please enter a number:");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // let num: i32 = input.trim().parse().expect("Please enter a number");
    // println!("You entered: {}", num);
    // return num;
    let num = 10;
    println!("You entered: {}", num);
    return num;
}
