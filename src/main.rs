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

enum Number_enum {
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

    println!("zero is {}", Number_enum::Zero as i32);
    println!("one is {}", Number_enum::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

fn test_var_bind() {
    let mut x = 2;
    {
        let x = "4";
    }

    x = 4;
}

fn casting() {
    let decimal = 22.8832_f32;

    let integer = decimal as u8;

    println!("Integer: {}", integer);

    let character = integer as char;

    println!("character: {}", character);

    println!("1000 as a u16 is: {:b}", 1000 as u16);

    let num = 1000;
    println!("1000 as a u8 is : {:b}", num as u8);

    println!("  -1 as a u8 is : {:b}", (-1i8) as u8);

    println!("1000 mod 256 is : {:b}", 1000 % 256);

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {:b} ({})", 128 as i16, 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    let num: i16 = 128;
    println!(" 128 as a i8 is : {:b} ({})", num as i8, num as i8);

    println!("127={:b}", 127_i8);
    println!("-128={:b}", -128_i8);

    println!("255={:b}", 255_u8);
    println!("0={:b}", 0_u8);

    println!("255= {}", 127_u8 as i8);
    println!("0={:b}", 0_u8 as i8);

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number is: {}", self.value)
    }
}
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn conversion() {
    let s = "Test";
    let myString = String::from(s);
    let b = myString.into_boxed_str();
    let ptr = b.as_ptr();
    println!("b:{:?}", ptr);
    let ref_b = b.as_ref();
    println!("s:{:?}", ref_b);

    let number = Number::from(34_i32);
    println!("{}", number);

    let n = 5;
    let num: i32 = n.into();
    println!("My number is {:?}", num);

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    println!("{:?}", EvenNumber::try_from(5));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

fn expression() {
    let x0 = 2;
    let sum = {
        let x = 20_i8;
        let y = 12;
        x + y + x0
    };

    println!("Sum: {:?}", sum)
}

fn flowControl() {
    let mut count = 0;
    loop {
        count += 1;

        if count % 2 == 0 {
            continue;
        }

        println!("Count: {}", count);

        if count > 6 {
            break;
        }
    }

    let mut optional = Some(20);

    match optional {
        Some(i) => {
            println!("Number: {:?}", i);
        }
        _ => {
            println!("Not present");
        }
    }

    if let Some(i) = Some(20) {
        println!("Number is indeed: {}", i);
    }

    while let Some(i) = optional {
        if i == 25 {
            println!("Number is: {}", i);
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

fn isDivisibleBy(lhs: u32, rhs: u32) -> bool {
    if (rhs == 0) {
        return false;
    }
    lhs % rhs == 0
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, p: &Point) -> f32 {
        let dx = self.x - p.x;
        let dy: f32 = self.y - p.y;
        (dx * dy + dy + dy).sqrt()
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

fn functions() {
    println!("isD: {:?}", isDivisibleBy(20, 4));
    println!("isD: {:?}", isDivisibleBy(20, 3));

    let mut point = Point::new(22.2, 32.3);
    let mut origin = Point::origin();

    println!("Distance: {:?}", point.distance(&origin));

    point.translate(3.0, -2.0);
    println!("Point: {:?}", point);

    println!("Distance: {:?}", point.distance(&origin));

    let x = 20;

    let fun1 = |i: i32| -> i32 { i + 1 + x };

    let fun2 = |i| i + 1 + x;

    let i = 1;
    println!("Inc1: {:?}", fun1(i));
    println!("Inc2: {:?}", fun2(i));

    let one = || 1;
    println!("One: {:?}", one());

    let color = "Green";

    let print = || println!("Color: {:?}", color);

    print();

    let _reborow = &color;
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("Count: {:?}", count);
    };

    inc();

    inc();

    let reborrow = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("Movable: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&2));
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I am function");
}

fn functions2() {
    let x = 30;

    println!("x: {:?}", x);

    let y = apply_to_3(|x| x + 20);

    println!("y: {:?}", y);

    let greeting = "Hello";
    let mut farewel = "Goodby".to_owned();

    let diary = || {
        println!("I said {}", greeting);

        farewel.push_str("!!!!!");
        println!("Than I screemed {}", farewel);

        println!("Than I sleep");

        mem::drop(farewel);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    let closure = || println!("I am closure");

    call_me(function);
    call_me(closure);
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a text: {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a text: {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn functions3() {
    let x = create_fn();
    x();

    let mut y = create_fn_mut();
    y();

    let z = create_fn_once();
    z();

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    println!("2 in v1: {}", v1.iter().any(|&x| x == 2));
    println!("2 in v2: {}", v2.iter().any(|&x| x == 2));

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    println!("2 in v1: {}", a1.iter().any(|&x| x == 2));
    println!("2 in v2: {}", a2.iter().any(|&x| x == 2));

    let mut iter1 = v1.iter();
    let mut into_iter = v2.into_iter();

    println!("Find 2 in v1: {:?}", iter1.find(|&&x| x == 2));

    println!("Find 2 in v1: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in v1: {:?}", array1.iter().find(|&&x| x == 2));

    println!("Find 2 in v1: {:?}", array2.into_iter().find(|&&x| x == 2));

    let index_of_first_even_number = array1.iter().position(|x| x % 2 == 0);

    println!(
        "index_of_first_even_number:  {}",
        index_of_first_even_number.unwrap()
    );
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn foo() -> () {
    ()
}

fn higherOrder() {
    let upper = 1000;
    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;
        if n_squared > upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("Sum1: {:?}", acc);

    let sum2 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| n_squared % 2 == 1)
        .fold(0, |acc, n_squared| acc + n_squared);

    println!("Sum2: {:?}", sum2);

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}

struct S;

struct GenericVal<T>(T);

impl GenericVal<i32> {}

impl GenericVal<S> {}

impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn generics() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}

fn create_box() {
    let _box = Box::new(3i32);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn scoping() {
    /* create_box();

    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
    }

    let x = ToDrop;
    {
        let y = ToDrop;
    }*/

    let x = 5u32;
    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    let mut b = a;
    *b = 30i32;
    
    //destroy_box(b);
    println!("{}", b);
}

fn main() {
    codewars::cw01::run();

    // literals_operator();

    //tuple_activity();

    //arrays_slices();
    // struct_test();
    //test_enum();
    //test_var_bind();
    //casting();

    //conversion();
    //expression();
    //flowControl();
    // functions();
    //functions2();
    //functions3();

    //higherOrder();

    //generics();

    scoping();
}
