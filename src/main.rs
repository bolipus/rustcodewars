use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::str::FromStr;

mod codewars;

fn literals_operator() {
    println!("1 + 2 = {}", 1i32 + 2);

    println!("0011 ^ 0110 = {:04b}", 0b0011u32 ^ 0b0110u32);

    println!("0011 << 2 = {:04b}", 0b0011u32 << 2);

    println!("0011 >> 2 = {:04b}", 0b0011u32 >> 2);

    println!("0o7 << 1 = {:04o}", 0o7u32 << 1);

    println!("0x7 << 1 = {:04x}", 0x7u32 << 1);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (x, y) = pair;
    (y, x)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

fn tuple_activity() {
    println!("{:?}", reverse((20i32, false)));
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);

    println!("Transpose:\n{}", transpose(matrix));
}

fn analyze_slice(slice: &[Matrix]) {
    println!("first element of the slice: \n{}", slice[0]);
    println!("the slice has {} elements.", slice.len());
}

fn arrays_slices() {
    let x: [Matrix; 2] = [
        Matrix(10.2f32, 2.1f32, 3.1f32, 4.5f32),
        Matrix(5.2f32, 6.1f32, 2.1f32, 8.5f32),
    ];

    println!("Array occupies: {:?} bytes", mem::size_of_val(&x));

    analyze_slice(&x[1..2]);
}

#[derive(Debug, Copy, Clone)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Person<'a> {
    fn changeName(&mut self, new_name: &'a str) -> &'a Person {
        self.name = new_name;
        self
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Triangle {
    top_left: Point,
    bottom_right: Point,
}

struct Unit;

struct Pair(i32, i32);

fn square(point: Point, val: f32) -> Triangle {
    Triangle {
        bottom_right: Point {
            x: point.x + val,
            y: point.y - val,
        },
        top_left: point,
    }
}

fn struct_test() {
    let mut person = Person {
        name: "Peter",
        age: 30u8,
    };
    let new_name = "Janez";
    let changedPerson = person.changeName(new_name);

    println!("Person: {:?}", changedPerson);

    let point = Point { x: 20.2, y: 30.3 };

    println!("Point: {:?}", point);

    let bottom_right = Point { x: 10.2, ..point };

    println!("bottom_right: {:?}", bottom_right);

    let top_left = Point { x: 2.2, y: 5.3 };

    let triangle = Triangle {
        bottom_right: bottom_right,
        top_left: top_left,
    };
    println!("{:?}", triangle);

    let Triangle {
        bottom_right: Point { x: x1, y: y1 },
        top_left: Point { x: x2, y: y2 },
    } = triangle;

    println!("x1: {}, y1: {}, x2: {}, y2:{}", x1, y1, x2, y2);

    println!("Area:{}", (x2 - x1) * (y2 - y1));

    let unit = Unit;

    let pair = Pair(20, 30);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(x, y) = pair;

    println!("pair contains {:?} and {:?}", x, y);

    println!("{:?}", square(point, 20.2));
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

use crate::WebEvent::*;

impl WebEvent {
    fn run(&self, m: i32) -> String {
        match self {
            PageLoad => return format!("Page loaded in {} seconds.", m),
            PageUnload => format!("PageUnloaded"),
            KeyPress(c) => format!("KeyPressed: {}", c),
            Paste(s) => format!("Pasted: {}", s),
            Click { x, y } => format!("Clicked on position: {}, {}", x, y),
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        PageLoad => println!("PageLoaded"),
        PageUnload => println!("PageUnloaded"),
        KeyPress(c) => println!("KeyPressed: {}", c),
        Paste(s) => println!("Pasted: {}", s),
        Click { x, y } => println!("Clicked on position: {}, {}", x, y),
    }
}

enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn test_enum() {
    let pressed = KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = Paste("my text".to_owned());
    let click = Click { x: 20, y: 80 };
    let load = PageLoad;
    let unload = PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let loadSec = WebEvent::PageLoad;
    println!("{}", &loadSec.run(20));

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

fn main() {


    println!("{}", codewars::cw01::reverse_words2("To je test"));

    /* println!("{}", duplicate_encode("Success"));

    println!("{}", reverse_words2("To je test"));*/

    /* let b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let c = vec!["A", "B", "C", "D"];

    println!("{}", stock_list(b, c));*/

    /*match race(720, 850, 70) {
         Some(val) => println!("{:?}", val),
         None => println!("")
    };*/

    /* let mut x = vec![0.0, 0.23, 0.46, 0.69, 0.92, 1.15, 1.38, 1.61];
    let mut s = 20;

    println!("{}", gps(s, x));*/

    /* println!("{}", good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"));
    println!("{}", good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"));
    println!("{}", good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"));*/

    // literals_operator();

    //tuple_activity();

    //arrays_slices();
    // struct_test();
    //test_enum();
}
