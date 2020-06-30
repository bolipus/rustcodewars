use std::collections::HashMap;

fn reverse_words(input: &str) -> String {
    let v: Vec<&str> = input.split(" ").collect();
    let mut output: String = String::from("");

    for x in v {
        let word = x.trim();
        if word.len() > 0 {
            output += " ";
            output += &word.chars().rev().collect::<String>();
        } else {
            output += " ";
        }
    }
    return String::from(output.trim());
}

fn reverse_words2(input: &str) -> String {
    String::from(input)
        .split(" ")
        .map(|sub| sub.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn duplicate_encode(word: &str) -> String {
    let mut map: HashMap<char, i8> = HashMap::new();
    let s = String::from(word.to_lowercase());
    for ch in s.chars() {
        let count = map.entry(ch).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    s.chars()
        .map(|ch| match *map.get(&ch).unwrap() {
            1 => '(',
            _ => ')',
        })
        .collect()
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut map: HashMap<char, i8> = HashMap::new();
    for s in list_art {
        let item: Vec<&str> = s.split(" ").collect();
        let cat: char = item[0].chars().next().unwrap();
        let count: &mut i8 = map.entry(cat).or_insert(0);
        let num: i8 = std::str::FromStr::from_str(item[1]).unwrap();
        *count += num;
    }

    return String::from(" ");
}

fn main() {
    println!("{}", duplicate_encode("Success"));

    println!("{}", reverse_words2("To je test"));

    let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let mut c = vec!["A", "B", "C", "D"];

    stock_list(b, c);
}
