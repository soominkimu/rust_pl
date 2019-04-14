// Mozilla, The Rust Programming Language: https://doc.rust-lang.org/book/index.html
// Rust by Example: https://doc.rust-lang.org/rust-by-example/
//
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    No
}

fn tc(clr: Color) -> &'static str {  // Lifetime Elision
    match clr {
        Color::Red    => "\x1B[0;31m",
        Color::Green  => "\x1B[0;32m",
        Color::Yellow => "\x1B[0;33m",
        Color::Blue   => "\x1B[0;34m",
        Color::No     => "\x1B[0m",
    }
}

fn print_chap(n: i32, s: &str) {  // string slice, an immutable view of a string
    println!("{}***{} Chap.{}{}{} {}{}",
             tc(Color::Red), tc(Color::Blue), tc(Color::Yellow), n, tc(Color::Green), s, tc(Color::No));
}

fn intro() {
    // https://theasciicode.com.ar/extended-ascii-code/black-square-ascii-code-254.html
    fn printline(c_left: char, c: char, c_right: char, count: u32) {
        print!("{}", c_left);
        let mut x = 0u32;
        loop {
            print!("{}", c);
            x += 1;
            if x == count { break; }
        }
        println!("{}", c_right);
    }
    const LEN : u32 = 37;
    const BAR_H : char = '═';
    const BAR_V : char = '║';
    let mut y = 0u32;

    print!("{}", tc(Color::Blue));
    printline('╔', BAR_H, '╗', LEN);
    loop {
        if y == 1 {
            println!("{}{}    The {}Rust{} Programming Language    {}{}",
                     BAR_V, tc(Color::Red), tc(Color::Yellow), tc(Color::Red), tc(Color::Blue), BAR_V);
        } else {
            printline(BAR_V, ' ', BAR_V, LEN);
        }
        y += 1;
        if y == 3 { break; }
    }
    printline('╚', BAR_H, '╝', LEN);

    for j in [30, 90].iter() {
        for i in 1..8 {
            print!("\x1B[0;{}m█\x1B[0m", j+i);
        }
    }
    println!();
}

fn chap02() {
    print_chap(2, "Programming a Guessing Game");

    use std::io;
    use std::cmp::Ordering;
    use rand::Rng;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // :: associated function (static method)

        io::stdin().read_line(&mut guess)    // mutable
            .expect("Failed to read line");  // io::Result enumerations
        guess = secret_number.to_string();   // DEBUG: just to skip

        let guess: u32 = match guess.trim().parse() { // shadow the previous value
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);  // {} placeholder

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}

fn chap03() {
    print_chap(3, "Common Programming Concepts");
    // Listing 3-5: Looping through each element of a collection using a for loop
    let a = [10, 20, 30, 40, 50];

    print!("the value is: ");
    for element in a.iter() {
        print!("{},", element);
    }
    println!();
}

fn chap04() {
    print_chap(4, "Understanding Ownership");
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
}

fn chap05() {
    print_chap(5, "Using Structs to Structure Related Data");

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
}

fn chap06() {
    print_chap(6, "Enums and Pattern Matching");

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

    println!("Value = {},{},{},{}",
        value_in_cents(Coin::Penny),
        value_in_cents(Coin::Nickel),
        value_in_cents(Coin::Dime),
        value_in_cents(Coin::Quarter));
}

fn chap08() {
    print_chap(8, "Common Collections");

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
        print!("{},", i);
    }
    println!();

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
}

fn chap09() {
    print_chap(9, "Error Handling");
    //let v_panic = vec![1,2,3];
    //v_panic[99];  // to panic
    // run $ RUST_BACKTRACE=1 cargo run

    use std::fs::File;
    use std::io::ErrorKind;

    use std::io;
    use std::io::Read;

    const HELLO_TXT: &str = "hello.txt";

    // Propagating Errors, Listing 9-6
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open(HELLO_TXT);

        let mut f = match f {
          Ok(file) => file,
          Err(e)   => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
          Ok(_)  => Ok(s),
          Err(e) => Err(e)
        }
    }

    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut f = File::open(HELLO_TXT)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_username_from_file_chained() -> Result<String, io::Error> {
        let mut s = String::new();

        let bytes : usize = File::open(HELLO_TXT)?.read_to_string(&mut s)?;
        println!("bytes: {}", bytes);

        Ok(s)
    }

    use std::fs;

    fn read_username_from_file_shortest() -> Result<String, io::Error> {
        fs::read_to_string(HELLO_TXT)
    }

    // Listing 9-10: A Guess type that will only continue with values between 1 and 100
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess {
                value
            }
        }
        //pub fn value(&self) -> i32 {
        //    self.value
        //}
    }

    let _f1 = File::open(HELLO_TXT);

    let _f1 = match _f1 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(HELLO_TXT) {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    let _f = File::open(HELLO_TXT).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(HELLO_TXT).unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    //let _f2 = File::open("hello.txt").unwrap();
    let _f2 = File::open(HELLO_TXT).expect("Filed to open hello.txt");

    println!("{}", read_username_from_file().unwrap());
    println!("{}", read_username_from_file_short().unwrap());
    println!("{}", read_username_from_file_chained().unwrap());
    println!("{}", read_username_from_file_shortest().unwrap());

    let n910 = Guess::new(50);
    println!("Guess: {}", n910.value);
}

fn chap10() {
    print_chap(10, "Generic Types, Traits, and Lifetimes");

    fn print_largest_num(n: i32) {
        println!("The largest number is {}", n);
    }

    fn print_largest_char(c: char) {
        println!("The largest char is {}", c);
    }

    // Removing Duplication by Extracting a Function
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    print_largest_num(largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    print_largest_num(largest);

    fn largest_f(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_f(&number_list);
    print_largest_num(result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_f(&number_list);
    print_largest_num(result);

    // Genereic Data Types
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    print_largest_num(result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    print_largest_char(result);

    /*
    // Listing 10-5: A definition of the largest funciton that uses generic type parameters but doesn't compile yet
    fn alargest<T>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {  // `T` might need a bound for `std::cmp::PartialOrd`
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = alargest(&number_list);
    print_largest_num(result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = alargest(&char_list);
    print_largest_char(result);
    */
}

fn main() {
    intro();
    chap02();
    chap03();
    chap04();
    chap05();
    chap06();

    chap08();
    chap09();
    chap10();
}
