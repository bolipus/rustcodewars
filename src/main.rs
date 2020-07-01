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
    let mut map: HashMap<char, i32> = HashMap::new();
    for s in list_art {
        let item: Vec<&str> = s.split(" ").collect();
        let c: char = item[0].chars().next().unwrap();
        let num: i32 = std::str::FromStr::from_str(item[1]).unwrap();
        let count = map.entry(c).or_insert(0);
        *count += num;
    }

    if map.len() == 0 {
        return String::from("");
    }
    let mut result = String::from("");

    for (index, cat) in list_cat.iter().enumerate() {
        let c: char = cat.chars().next().unwrap();
        let count = match map.get(&c) {
            Some(count) => *count,
            None => 0i32,
        };

        result.push_str(format!("({}:{})", c, count).as_str());
        if index < list_cat.len() - 1 {
            result.push_str(" - ");
        }
    }

    return result;
}

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    } else {
        let mut seconds = g * 3600 / (v2 - v1);
        let hours = seconds / 3600;
        seconds -= hours * 3600;
        let minutes = seconds / 60;
        seconds -= minutes * 60;

        Some(vec![hours, minutes, seconds])
    }
}

fn gps(s: i32, x: Vec<f64>) -> i32 {
    /*let speed = (|dx:f64,s| (3600f64 * dx)/s );
    let mut maxSpeed = 0f64;
    for i in 1..x.len() {
        let dx = x[i]-x[i-1];
        let sp = speed(dx,s as f64);
        if sp > maxSpeed{
            maxSpeed = sp;
        }
    }
    return maxSpeed.floor() as i32;*/
    x.iter()
        .zip(x.iter().skip(1))
        .map(|(a, b)| (b - a) * 3600.0 / (s as f64))
        .fold(0.0, f64::max)
        .floor() as i32
}

fn main() {
    /* println!("{}", duplicate_encode("Success"));

    println!("{}", reverse_words2("To je test"));*/

    /* let b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let c = vec!["A", "B", "C", "D"];

    println!("{}", stock_list(b, c));*/

    /*match race(720, 850, 70) {
         Some(val) => println!("{:?}", val),
         None => println!("")
    };*/

    let mut x = vec![0.0, 0.23, 0.46, 0.69, 0.92, 1.15, 1.38, 1.61];
    let mut s = 20;

    println!("{}", gps(s, x));
}
