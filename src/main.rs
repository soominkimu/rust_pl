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
}

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
