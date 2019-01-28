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
    let word2 = first_word2(&t2[..]);
    let t3 = "hello world";
    let word2 = first_word2(t3);
    //t2.clear();
    println!("word2 = {}", word2);
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
