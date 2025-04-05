use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let score=80;
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
    let arr=[2,3,4,5,6,7,8,9,10];
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

    let y1=scores.get("Alice").unwrap();
    println!("Alice's score: {}", y1);
    let y2=scores.get("Bob").unwrap();
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

}

fn get_num()->i32{
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
