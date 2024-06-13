static MY_STATIC: i32 = 42; //不能修改
static mut MY_STATIC_MUT: i32 = 33;//mut 可修改

enum Color {
    Red,
    Green,
    Black,
    White,
    Gray,
}

fn show_color(color: Color) -> &'static str {
    return match color {
        Color::Red => { "Red" }
        Color::Green => { "Green" }
        Color::Black => { "Black" }
        Color::White => { "White" }
        _ => "UnKnown Color",
    }
    // match color {
    //     Color::Red => println!("Red"),
    //     Color::Green => println!("Green"),
    //     Color::Black => println!("Black"),
    //     _ => println!("UnKnown Color"),
    // }
}

fn main() {
    // println!("Hello, world!");
    // ex2();
    // ex3();
    // ex4();
    // ex8();
    // learn_const();
    // learn_static();
    // learn_basic();
    // learn_tuple_array();
    println!("{}", show_color(Color::Gray));
}

fn learn_tuple_array() {
    //array 固定长度的同构集合 tuple是固定长度的异构集合
    let a = [1, 2, 3];
    // let b: [* const str; 3] = ["a", "b", "c"];
    println!("{:?},len={},first={}", a, a.len(), a[0]);


    let t = (0, "nine", 3.14);
    println!("tuple {},{},{}", t.0, t.1, t.2);
    let mut t2 = (0, "nine", 3.14);
    println!("tuple2 {},{},{}", t2.0, t2.1, t2.2);
    t2.1 = "f";
    println!("tuple2 {},{},{}", t2.0, t2.1, t2.2);

    let t3 = ();
    println!("{:?}", t3)
}

fn learn_basic() {
    // 10进制数字 11 对应的 16进制,8进制,2进制
    let a1: i32 = 11;
    let a2 = 0x11;
    let a3 = 0o11;
    let a4 = 0b11;

    println!("{a1},{a2},{a3},{a4}");
    println!("{} \r\n{} \r\n{}\r\n{}\r\n{}", u32::MAX, u32::MIN, i32::MAX, i32::MIN, usize::MAX);

    let char_c = 'C';
    let emo_char = '🚀';
    print!("{},{}", char_c, emo_char);
}

fn learn_static() {
    println!("{}", MY_STATIC);
    unsafe {
        MY_STATIC_MUT = 22;
        println!("{}", MY_STATIC_MUT);
    }
    // println!("{}", MY_STATIC_MUT); //只能在unsafe 作用域内打印
}

fn learn_const() {
    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR; //编译时SECOND_HOUR 已确定了
    println!("{SECOND_DAY}");
    {
        //作用域可见
        const SE: usize = 1_000;
        println!("{SE}"); //此处SE在作用域内，可见，所以正常打印
    }
    // println!("{SE}"); //此处SE 不在作用域内，所以打印出错
}

fn ex8() {
    let x;
    {
        let input = String::from("this is the input");
        x = foo(&input);
    }
    println!("{x}");
}

fn foo<'a>(inp: &'a str) -> &'static str {
    "abc"
    // let s :&'static str = "abc";
}

// #[derive(Debug)]
// struct SplitStr<'a, 'b> {
//     start: &'a str,
//     end: &'b str,
// }

fn split<'a, 'b>(text: &'a str, delimiter: &'b str) -> Option<SplitStr<'a>> {
    let (start, end) = text.split_once(delimiter)?;
    Some(SplitStr { start, end })
}

#[derive(Debug)]
struct SplitStr<'a> {
    start: &'a str,
    end: &'a str,
}

/// 任何引用都有一个生命周期,且需要为使用引用的函数或结构体指定生命周期参数.函数参数或方法参数中的生命周期被称为输入生命周期(input lifetime),返回的生命周期称为输出生命周期(output lifetime)
/// 在没有显示标注的情况下,编译器使用了3种规则来计算引用生命周期.第一条规则作用域输入生命周期,第二,三条作用于输出生命周期.当编译器检查完这三条规则后仍有无法计算出生命周期的引用时,编译器就会停止运行并抛出错误.
/// 这些规则不但对fn定义生效,也对impl代码块生效.
/// R1:每一个引用参数都会拥有自己的生命周期参数
/// R2:当只存在一个输入生命周期参数时,这个生命周期会被赋予给所有输出生命周期参数
/// R3:当拥有多个输入生命周期参数,而其中一个是&self或&mut self时,self的生命周期会被赋予给所有的生命周期参数.
/// 生命周期在函数上的省略规则: 1. 函数参数 2.  3.
/// trait object 省略规则
///  'static 生命周期省略规则


fn ex5() {
    usage_1();
    usage_2();
}

fn identity(a: &i32) -> &i32 {
    a
}

fn usage_1() {
    let x = 4;
    let x_ref = identity(&x);
    assert_eq!(*x_ref, 4);
}

fn usage_2() {
    let mut x_ref: Option<&i32> = None;
    let x = 7;//让x活到 花括号结束
    {
        // let x = 7;
        x_ref = Some(identity(&x));
    }
    assert_eq!(*x_ref.unwrap(), 7);
}

fn ex4() {}

fn only_if_greater<'num, 'gt>(number: &'num i32, greater_than: &'gt i32) -> Option<&'num i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}

fn ex2() {
    let mut vec: Vec<_> = vec![];
    vec.push(42);
}

fn ex3() {
    let a = 1;
    let my_num = complex_function(&a);
    println!("{my_num}")
}

fn complex_function(a: &i32) -> &i32 {
    let b = 2;
    // max_of_refs(a, &b)
    max_of_refs1(a, &b)
}

fn max_of_refs<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a > *b {
        a
    } else {
        b
    }
}

fn max_of_refs1<'a, 'b>(a: &'a i32, b: &'b i32) -> &'a i32 {
    a
}