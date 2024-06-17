use std::num::ParseIntError;

/// map 对一个集合中每个元素应用一个函数,且返回包含结果的集合
/// filter  用于过滤集合中元素,,根据一个谓词函数的返回值.
/// fold 也叫reduce,可用于迭代集合中每个元素,并将它们累积到一个单一的结果中.

pub fn mul_twice(func: fn(i32) -> i32, x: i32) -> i32 {
    func(func(x))
}

pub fn mul(x: i32) -> i32 {
    x * x
}

pub fn add(x: i32) -> i32 {
    x + x
}

///  Rust中错误分类: 1 Recoverable error:有返回类型 Result /Option
///                2 Unrecoverable type:没有返回类型,直接崩溃 panic macro 将终止当前线程
///  Result 枚举类型 Ok,Err       Option也是枚举类型 Some None,表示一个可能为空的值.
/// pub enum Result<T,E>{     pub enum Option<T>{
///     Ok(T),                       None,
///     Err(E),                      Some(T),
/// }                          }
///  panic! 当程序遇到无法继续执行的错误时,使用panic!引发恐慌,导致程序立即终止,并显示一条错误信息

pub fn fun_error() {
    //Result
    let x = 1.0f64;
    let y = 2.0f64;
    let y = 0.0;
    match divide(x, y) {
        Ok(number) => println!("{number}"),
        Err(err) => println!("{err}"),
    }

    //Option
    let arr = [1, 2, 3, 4, 5, 6, 7];
    match find_element(&arr, 5) {
        None => { println!("None") }
        Some(index) => { println!("found in {index}") }
    }

    match find_element(&arr, 9) {
        None => { println!("None") }
        Some(index) => { println!("found in {index}") }
    }

    //panic  数组越界
    let arr = vec![1, 2, 3, 4, 5];
    arr[8];
}

fn find_element(array: &[i32], target: i32) -> Option<usize> {
    for (index, element) in array.iter().enumerate() {
        if *element == target {
            return Some(index);
        }
    }
    None
}


fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("cannot be zero"));
    }
    Ok(a / b)
}

/// unwrap ?用于简化Result或Option类型的错误传播,它只能用于返回Result或Option的函数中,且在函数内部可以像使用unwrap()一样访问Ok或Some的值,
/// 如果是Err或None则会提前返回.
pub fn func_unwrap() {
    let result_ok: Result<i32, &str> = Ok(200);
    println!("{}", result_ok.unwrap());
    // let result_err: Result<i32, &str> = Err("0xFF");
    // println!("{:?}", result_err.unwrap()); //这里会直接panic
    //
    let vec = vec![1, 3, 5, 7, 9];
    match find_1_even(vec.clone()) {
        None => { println!("Not found even in {:?}", vec) }
        Some(number) => { println!("first even = {number}") }
    }

    match parse_numbers("d") {
        Ok(i) => { println!("parsed {}", i) }
        Err(err) => { print!("parsed failed {}", err); }
    }
}

fn find_1_even(vec: Vec<i32>) -> Option<i32> {
    Some(*vec.iter().find(|&num| num % 2 == 0)?)
}

fn parse_numbers(input: &str) -> Result<i32, ParseIntError> {
    Ok(input.parse::<i32>()?)
}