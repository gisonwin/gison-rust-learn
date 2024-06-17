use crate::err_func::*;
use crate::higher_func::*;

pub mod higher_func;
mod err_func;

enum Flavor {
    Spicy,
    Sweet,
    Fruity,
}

struct Drink {
    price: f64,
    flavor: Flavor,
}

impl Drink {
    //关联变量
    const MAX_PRICE: f64 = 10.0;
    //方法
    fn buy(&self) {
        if self.price > Self::MAX_PRICE {
            println!("I am poor");
            return;
        }
        println!("buy it success")
    }
    //关联函数
    fn new(price: f64, flavor: Flavor) -> Self {
        Drink {
            flavor,
            price,
        }
    }
}


fn show_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Spicy => { println!("Spicy") }
        Flavor::Sweet => { println!("Sweet") }
        Flavor::Fruity => { println!("Fruity") }
    }
    println!("{}", drink.price)
}

fn learn_struct() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 3.5,
    };
    println!("{}", sweet.price);

    show_drink(sweet);
    //sweet 所有权丢失了
    // println!("{}", sweet.price);
    let spicy = Drink::new(12.0, Flavor::Spicy);
    spicy.buy()
}

fn main() {
    // learn_struct();
    // learn_number();
    // learn_trait();
    // learn_func();
    // learn_hf();
    // fun_error();
    // func_unwrap();
    func_errs(MyError { detail: "自定义error detail".to_owned() });
    // let rs = func_errs_char();
}

fn learn_hf() {
    let result = mul_twice(mul, 4);
    println!("{}", result);
    let state = "中华人民共和国";
    println!("{}", state.len());
    // let state = String::from_utf8()
    let res = mul_twice(add, 4);
    println!("{}", res);
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let collect = numbers.iter().map(|&x| x + x).collect::<Vec<_>>();
    println!("{:?}", collect);
    println!("{:?}", numbers);
    let even = numbers.clone().into_iter().filter(|&x| x % 2 == 0).collect::<Vec<i32>>();
    let odd = numbers.clone().into_iter().filter(|&x| x % 2 != 0).collect::<Vec<_>>();
    println!("even={:?},odd={:?}", even, odd);
    //reduce
    let sum = numbers.clone().iter().fold(0, |acc, &x| acc + x);
    println!("{}", sum);
}


fn add_two_var(a: i32, b: i32) -> i32 {
    a + b
}

fn learn_func() {
    let a = 1;
    let b = 2;
    let c = add_two_var(a, b);
    println!("c:{}", c);


}

/// stack && heap
/// stack 后进先出,操作高效,函数作用域就作用于栈上,stack上存储所有数据都必须具有已知固定大小
/// heap  长度不确定,当你请求把数据放在heap上时,先请求空间,然后返回一个指针,指向该位置的地址.
/// Box指针 ,智能指针,提供对heap分配内存的所有权,允许你将数据存储在heap上而不是stack上,且复制或移动时保持对数据的唯一拥有权.使用Box可避免一些内存管理问题如悬垂指针和重复释放
///  1. 所有权转移  2 释放内存 3 解引用 4 构建递归数据结构
///
/// Copy && Clone
/// Move:所有权转移
/// Clone:深拷贝
/// Copy: 是在Clone的基础上建立的market trait. 1.trait是一种定义共享行为的机制.Clone也是特质 trait.
/// 2 market trait 是一个没有任何方法的trait,它主要用于向编译器传递某些信息,以改变类型的默认行为.
///
///
///
/// Stack 1.基础类型 2 tuple ,array 3 struct,enum存储stack上,如果属性有String在堆上的数据类型会有指向堆的.
/// Heap : Box Rc String/Vec 等
/// 一般来说stack上的数据类型都默认Copy,但Struct等默认Move,需要Copy只需要设置数据类型实现Copy trait即可,或调用Clone函数(需实现Clone trait)

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Book {
    page: i32,
    rating: f64,
}


fn learn_trait() {
    let boxed_pointer = Box::new(Point { x: 10, y: 20 });
    println!("x:{},y:{}", boxed_pointer.x, boxed_pointer.y);

    let mut boxed_point = Box::new(32);
    println!("{}.{}", boxed_point, *boxed_point);
    *boxed_point += 10;
    println!("{}", *boxed_point);

    let v1 = vec![1, 2, 3, 4];
    // let v2 = v1; //v1所有权转移到v2,必须clone
    let v2 = v1.clone();
    println!("{:?},{:?}", v1, v2);

    let x = "string".to_string();
    // let y = x;
    let y = x.clone();
    println!("{:?},{:?}", x, y);

    //struct copy and clone
    let b1 = Book {
        page: 100,
        rating: 0.1,
    };
    let b2 = b1;
    println!("{:?},{:?}", b2, b1)
}


/// &self (self: &Self)  不可变引用
/// &mut self (self: &mut Self) 可变引用
/// self (self: Self)  Move

struct Counter {
    number: i32,
}

impl Counter {
    fn new(number: i32) -> Self {
        Counter { number }
    }
    //不可变引用
    fn get_number(&self) -> i32 {
        self.number
    }
    //可变引用
    fn change_number(&mut self, incr: i32) {
        self.number += incr;
    }
    // move
    fn move_number(self) {
        println!("move {}", self.number);
    }
    //combine
    fn combine_number(s1: Self, s2: Self) -> Self {
        Self {
            number: s1.number + s2.number,
        }
    }
}

fn learn_number() {
    let c1 = Counter::new(0);
    println!("{}", c1.get_number());
    println!("{}", c1.get_number());
    let mut c2 = Counter::new(0);
    c2.change_number(2);
    println!("c2=={}", c2.get_number());
    println!("c2=={}", c2.get_number());
    c2.move_number();
    //下面会报错,因为所有权转移到move_number里面去了
    // println!("c2=={}", c2.get_number());

    let c1 = Counter::new(11);
    let c2 = Counter::new(22);
    let c3 = Counter::combine_number(c1, c2);
    println!("c3=={}", c3.get_number());
}