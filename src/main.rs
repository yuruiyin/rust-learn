mod struct_test;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::ptr::null;
use crate::struct_test::test_struct;

fn main() {
    // println!("Hello, world!");
    // hello_world();
    // // 打印数字
    // println!("{}", add(2, 5));

    test_compound_type();
    test_function();
    test_loop();
    test_for();
    test_str();
    test_move_and_clone();
    test_references();
    test_slice();
    test_struct();
    test_guess_game();
}

// 测试slice
fn test_slice() {
    println!("test slice");
    let mut  s = String::from("Slice hello world");
    let slice_s = &s[..];
    let s1 = &s;
    println!("{}, {}", slice_s, s1);
    println!("{}", slice_s == s1);

    let arr = [1, 2, 3, 4, 5];
    let slice_arr = &arr[1..3];
    println!("slice_arr len: {}, slice_arr index 1: {}", slice_arr.len(), slice_arr[1])
}

// 测试引用
// 可变引用在自己作用域内，只能独立存在，即不能与其他可变引用或者不可变引用同时存在
// 不可变引用在自己作用域内可以与其他不可变引用同时存在
fn test_references() {
    println!("test references");
    let mut  s = String::from("hello");
    let ref_s = &s;
    let ref_s1 = &s;
    println!("{}, {}", ref_s, ref_s1);
    let mut_ref_s = &mut s;
    println!("{}", mut_ref_s)
}

// 测试字符串的移动和克隆
fn test_move_and_clone() {
    println!("test str move and clone");
    let str1 = String::from("Hello");
    let str2 = str1;
    // 将str1 赋值（实际上是移动）给str2之后，str1就被释放了，不能使用了。
    println!("{}", str2);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    let ss1 = "Hello1";
    let ss2 = ss1;
    println!("{}", ss1);

    let sss1 = String::from("one_str");
    takes_ownership(sss1);
    // println!("xxxx: {}", sss1)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn test_str() {
    println!("test str");
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s)
}

fn test_for() {
    println!("test for");
    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("{}", element);
    }

    for i in (1..4).rev() {
        println!("{}", i);
    }

}

fn test_loop() {
    println!("test loop");
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("{}", result)
}

// 猜数字游戏
fn test_guess_game() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_num}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// 测试复合类型
// 1. 元组（tuple）：元组中的每个元素的类型可以不一行
// 2. 数组（array）：数组中的每个元素的类型必须一样
fn test_compound_type() {
    let tup = (1, 0.5, "haha");
    let (x, y, z) = tup;
    println!("{z}");
    println!("{},{},{}", tup.0, tup.1, tup.2);

    let arr = [0, 1, 2, 3, 4];
    println!("{}", arr[2]);
    // println!("{}", arr[5]);

    let arr1 = ["xiaoming", "xiaohong", "xiaolinag"];
    let arr2 = [6; 3];
}

fn test_function() {
    let x = {
        let y = 3;
        y + 1
    };
    println!("表达式：{}", x)
}

fn hello_world() {
    println!("hello world1111!")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
    // return a + b;
}
