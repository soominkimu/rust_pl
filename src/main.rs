#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle {  // method
    fn area(&self) -> u32 {  // immutable borrow
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width  > other.width &&
        self.height > other.height
    }

    fn square(size: u32) -> Rectangle {  // associated function
        Rectangle { width: size, height: size }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None    => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let mut s = String::from("hello");
    change(&mut s);

    let mut t1 = String::from("hello world");
    let word = first_word(&t1);
    t1.clear();
    println!("word = {}", word);

    let t2 = String::from("hello world");
    let _word2 = first_word2(&t2[..]);
    let t3 = "hello world";
    let word2 = first_word2(t3);
    //t2.clear();
    println!("word2 = {}", word2);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {:?}",  rect1);
    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect sq {:?}", Rectangle::square(3));

    let five = Some(5);
    println!("plus_one(five) = {:?}", plus_one(five).unwrap());
    //println!("plus_one(None) = {:?}", plus_one(None).unwrap());  // panic!

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("Three!");
    }

    println!("Value = {}", value_in_cents(Coin::Penny));
    println!("Value = {}", value_in_cents(Coin::Nickel));
    println!("Value = {}", value_in_cents(Coin::Dime));
    println!("Value = {}", value_in_cents(Coin::Quarter));

    // Chap.8 Common Collections
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None        => println!("There is no third element."),
    }

    let first = &v[0];
    println!("The first element is {}", first);
    v.push(6);

    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

#[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("SpreadsheetCell row = {:?}", row);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let st1 = String::from("Hello, ");
    let st2 = String::from("world!");
    let st3 = st1 + &st2;  // st1 has been moved here and can no longer be used
    println!("st3 = {}", st3);

    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name  = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);

    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    let team_name1 = String::from("Blue");
    let score1 = scores1.get(&team_name1);
    println!("{}: {:?}", team_name1, score1);

    for (key, value) in &scores1 {
      println!("{}: {}", key, value);
    }

    // Updating a Hash Map
    scores1.insert(String::from("Blue"), 25);
    println!("{:?}", scores1);

    scores1.entry(String::from("Red")).or_insert(50);
    scores1.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores1);

    let text1 = "hello world wonderful world";
    let mut map1 = HashMap::new();
    for word in text1.split_whitespace() {
      let count = map1.entry(word).or_insert(0);  // returns a mutable reference (&mut V)
      *count += 1;
    }
    println!("{:?}", map1);

    //let v_panic = vec![1,2,3];
    //v_panic[99];  // to panic
    // run $ RUST_BACKTRACE=1 cargo run

    use std::fs::File;
    use std::io::ErrorKind;
    /*

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    */

    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}  // main

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {  // enumerate method returns a tuple, destructure it
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

