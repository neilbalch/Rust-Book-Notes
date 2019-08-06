// use std::vec;
use std::collections::HashMap;

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(100);

    let second: &i32 = &v3[1];
    println!("The second element is: {}", second);

    match v3.get(1) {
        Some(second) => println!("The third element is: {}.", second),
        None => println!("There is no third element."),
    }

    for element in &v3 {
        println!("\t{}", element);
    }

    for element in &mut v3 {
        *element *= 2;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14159),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

fn strings() {
    let mut s = String::new();

    let data = "this is some sample text";
    let data_str = data.to_string();
    let data_str2 = "this is some sample text".to_string();
    let data_str3 = String::from("this is some sample text");

    // Strings are UTF-8 encoded, so...
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Add text to the end of the string.
    s.push_str("some text");
    s.push('A');
    let s2 = s + " and other text"; // NOTE: s here is moved and no longer usable.

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // A OR B:
    let mut s = tic + "-" + &tac + "-" + &toe;
    // let mut s = format!("{}-{}-{}", tic, tac, toe)

    let mut len = String::from("Hola").len(); // 4.
    len = String::from("Здравствуйте").len(); // 24, not 12. UTF-8 scalars are 2 bytes each.

    let hello = "Здравствуйте";
    let s = &hello[0..4];       // First 4 bytes, translating to two letters: Зд
    // NOPE! This causes a runtime panic, as the first byte alone isn't
    // sufficient to describe a whole number of characters.
    // let s = &hello[0..1];

    for c in "नमस्ते".chars() {
        println!("\tchar: {}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("\tchar: {}", c);
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Red");
    let field_value = 100;
    scores.insert(field_name, field_value);
    // Now, field_name and field_value are invalid, they were taken into the
    // ownership of the hashmap.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // This is an Option<> value.

    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    scores.insert(String::from("Blue"), -10); // Overwrites the original value.
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50); // Only insert the item if it doesn't already exist.

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {   // Add each word only once, counting the number of repeats.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn int_stats(list: &Vec<i32>) -> (f64, i32, i32) {
    // Sort the numbers.
    let mut ordered_list = Vec::new();
    for element in list {
        let mut index = 0;
        for item in &ordered_list {
            if *item < element {
                index += 1;
            } else if *item <= element {
                break;
            }
        }

        ordered_list.insert(index, element);
    }

    let len = ordered_list.len();

    // Get the average.
    let mut mean = 0.0;
    for element in &ordered_list {
        mean += f64::from(**element);
    }
    mean /= len as f64;

    // Get the median.
    let mut median = 0;
    if len % 2 == 0 {
        match &ordered_list.get(len / 2) {
            Some(num) => median = i32::from(***num),
            None => (),
        }
    } else {
        let mut num1 = 0;
        let mut num2 = 0;
        match &ordered_list.get(len / 2) {
            Some(num) => num1 = i32::from(***num),
            None => (),
        }
        match &ordered_list.get((len / 2) + 1) {
            Some(num) => num2 = i32::from(***num),
            None => (),
        }
        median = (num1 + num2) / 2;
    }

    // Get the mode.
    let mut mode = 0;
    let mut current_mode_count = 0;
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for element in &ordered_list {
        let count = occurrences.entry(**element).or_insert(0);
        *count += 1;
    }
    for (key, value) in &occurrences {
        if *value > current_mode_count {
            mode = *key;
            current_mode_count = *value;
        }
    }

    (mean, median, mode)
}

fn main() {
    // vectors();
    // strings();
    // hashmaps();

    let list = vec![1, 4, 3, 2, 0, 5, 3, 2, 2];
    println!("{:?}", int_stats(&list));
}
