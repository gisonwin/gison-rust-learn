// #![feature(lazy_cell)]
// #![feauture(thin_box)]

// use std::boxed::ThinBox;
use std::mem::size_of;

///Rust最显著的原语之一是 ownership system,它允许你在没有锁的情况下管理内存访问。还提供一些并发编程的工具和标准库，比如线程，线程池，消息
///通讯(mpsc等),原子操作等.并发原语较多,本单介绍 Cow,beef::Cow,box,Cell,RefCell,OnceCell,LazyCell,LazyLock,RC这些称为
///容器类并发原语,主要基于它们的行为,对普通数据进行包装,以便提供其他更丰富的功能.

/// 1 cow (clone-on-write OR copy-on-write)缩写,是一种优化内存和提高性能的技术,通常应用在资源共享的场景,其基本思想是,当
/// 有多个调用者(callers)同时请求相同的资源时,都会共享同一份资源,直到有调用者试图修改资源内容时,系统才会真正复制一份副本出来给
/// 调用者,而其他调用者依然使用原来的资源. Rust中的String,Vec等类型就利用了COW.
/// let s1 = String::from("hello");
/// let s2 = s1;//S1,s2共享同一份内存
/// s2.push_str(" world");//s2进行push操作,此时系统复制一份新的内存给s2
/// 这样避免大量未修改的字符串,向量等的重复分配和复制,提高内存利用率和性能.
/// cow的优点:内存利用率高,只有进行写时才复制,读取性能高,多个调用者共享同一资源
/// cow的缺点:写时需要复制,有一定性能损失,实现较复杂.
///
/// 对于存在大量相同或相似资源共享情况,使用cow可以带来显著性能提升.标准库中std::borrow::Cow类型是一个智能指针,提供了写
/// 时克隆(clone-on-write)的功能:它可以封装并提供对借用数据的不可变访问,当需要进行修改或获取所有权时,可以惰性克隆数据.
/// Cow实现了Deref,意味着你可直接在其封装的数据上调用不可变方法.如果需要进行改变,则to_mut将获取一个对拥有的值的可变引用,必要时
/// 进行克隆.下面代码将origin字符串包装成一个cow,可把它borrowed成一个$str,也可以在cow调用$str方法,因为cow实现了Deref,
/// 可以自动解引用,比如直接调用len,into:
/// let origin ="hello world";
/// let mut cow = Cow::from(origin);
/// assert_eq!(cow,"hello world");
///
/// //cow can be borrowed as a str
/// let s:$str = $cow;
/// assert_eq!(s.len(),cow.len());
/// //cow can be converted to a String
/// let s:String  = cow.into();
/// 下面是一个写时clone的例子:将字符串的字符全部改成大写字母
/// //cow can be  borrowed as a mut str
/// let s:&mut str = cow.to_mut();
/// s.make_ascii_uppercase();
/// assert_eq!(s,"HELLO WORLD");
/// assert_eq!(origin,"hello world);
/// to_mut得到一个可变引用,一旦s有修改,它会从原始数据中clone一份,在克隆的数据上进行修改.
/// 如果你想在某些数据上实现copy-on-write/clone-on-write,可以考虑std::borrow::Cow.
/// beef库提供了一个更快,紧凑的Cow类型,使用方法和标准库的Cow使用类似.
///

pub fn beef_cow() {
    let borrowed: beef::Cow<str> = beef::Cow::borrowed("Hello");
    let owned: beef::Cow<str> = beef::Cow::owned(String::from("World"));
    let _ = beef::Cow::from("Hello");
    assert_eq!(format!("{} {}!", borrowed, owned), "Hello World!");

    const WORD: usize = size_of::<usize>();
    assert_eq!(size_of::<std::borrow::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
}

/// Cow::borrowed:借用已有资源
/// Cow::fom：从已有资源复制创建
/// Cow::owned：自己提供资源内容

///后半部分对比了标准库Cow和beef::Cow以及beef::lean::Cow所占用内存大小.对于str类型的Cow,标准库是占用三个WORD
/// ,beef::Cow一样占用3个WORD,而beef::lean::Cow只占用了两个WORD. cow-utils针对字符串的Cow做了优化,性能更好.



/// box
/// Box<T>通常简称为box,提供了Rust中最简单的堆分配形式.Box为这个分配提供了所有权,并在超出作用域时释放其内容,并确保它们不会分配超过isize::MAX字节的内存
/// let val:u8 = 5; let boxed:Box<u8>=Box::new(val);
/// 通过解引用把值从堆上移动到栈上 :let boxed:Box<u8> = Box::new(5); let val:u = *boxed;
/// 我们定义一个递归的数据结构,比如链表,不能使用list,因为list大小不固定,我们不知道该分配给它多少内存.
///#[derive(Debug)]
///enum List<T>{
///     Cons(T,list<T>),
///     Nil,
/// }
/// 此时应该用Box. #[derive(Debug)]
/// enum List<T>{
///     Cons(T,Box<List<T>>),
///     Nil,
/// }
/// let list:List<i32> = List::Cons(1,Box::new(List::Cons(2,Box::new(List::Nil))));
/// println!("{list:?}");
/// Rust还提供ThinBox,实验性的类型,是一个瘦指针,不管内部元素的类型是啥:
// fn thin_box_example() {
//     use std::mem::{size_of, size_of_val};
//
//     let size_of_ptr = size_of::<*const ()>();
//     let box_five = Box::new(5);
//     let box_slice = Box::<[i32]>::new_zeroed_slice(5);
//     assert_eq!(size_of_ptr, size_of_val(&box_five));
//     assert_eq!(size_of_ptr * 2, size_of_val(&box_slice));
//
//
//     // ThinBox::new(5);
// }

///  Cell,RefCell,OnceCell,LazyCell,LazyLock
///
/// Cell和RefCell是Rust中用于内部可变性(interior mutability)两个重要类型.Cell和RefCell都是可共享的可变容器.可共享的可变容器是为了以受控方式允许可变性,即使存在别名引用.
/// 且允许在单线程环境下以这种方式运行,但它们都不是线程安全的,因为没有实现Sync.
/// Cell<T>允许在不违反借用规则前提下,修改其包含的值:Cell中的值不再拥有所有权,只能通过get,set方法访问,set就去可在不获取可变引用的情况下修改Cell的值.适用于简单的单值容器,如整数或字符.
/// 创建一个Cell,赋值给变量x,x这里是不可变的,但我们能通过set方法修改它的值,且即使存在结x的引用y的时也可以修改它的值.
use std::cell::{Cell, OnceCell, RefMut};

fn cell_use() {
    let x = Cell::new(42);
    let y = &x;
    x.set(10);//可以修改
    println!("y:{:?}", y.get());//输出y=10.
}

/// RefCell<T>提供更灵活的内部可变性,允许在运行时检查借用规则,通过运行时借用检查来实现,通过borrow,borrow_mut方法进行不可变和可变借用,借用必须在作用域结束前归还,否则会panic.适用于包含多个字段的容器.
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::borrow::Borrow;

fn ref_cell_sample() {
    let x = RefCell::new(42);
    {
        let y = x.borrow();
        //在这个作用域内,只能获得不可变引用
        println!("y:{:?}", *y.borrow());
    }
    {
        let mut z = x.borrow_mut();
        //此作用域内,可获得可变引用
        *z = 10;
    }
    println!("x:{:?}", x.borrow().deref());
}

/// OnceCell是Rust标准库,用于提供一次性写入的单元格,它允许在运行时将值放入单元格,但只允许一次,一旦值被写入,进一步的写入尝试将被忽略.主要特点和用途:
/// - 一次性写入:OnceCell确保其内部值只能被写入一次.一旦值被写入,后续写入操作将被忽略.
/// - 懒初始化:OnceCell支持懒初始化,即只有在需要时才会进行初始化,在需要运行时确定何时初始化值的场景下很有用.
/// - 线程安全:OnceCell提供了线程安全的一次性写入,在多线程环境中,它确保只有一个线程能够成功写入值,而其他线程的写入尝试将被忽略.
/// 演示OnceCell用法,未初始化时,获取它的值是None,一旦初始化为Hello,world,它的值就固定下来了:
pub fn once_cell_example() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none()); //true

    let value: &String = cell.get_or_init(|| "Hello,World!".to_string());
    assert_eq!(value, "Hello,World!");
    assert!(cell.get().is_some());//true
}


///Rust中实现懒(惰性)初始化效果时提供了LazyCell,LazyLock.但处于不稳定状态,需要设置#![feature(lazy_cell)]使能它.
// fn lazy_cell_example() {
//     #![feature(lazy_cell)]
//     use std::cell::LazyCell;
//     use std::collections::HashMap;
//     use std::sync::LazyLock;
//     let lazy: LazyCell<i32> = LazyCell::new(|| {
//         println!("initializing");
//         46
//     });
//     println!("ready");
//     println!("{}",
//              *lazy); // 46
//     println!("{}",
//              *lazy); // 46
//     static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
//             println!("lazy lock initializing");
//             let mut m = HashMap::new();
//             m.insert(13, "Spica".to_string());
//             m.insert(74, "Hoyten".to_string());
//             m
//         });
// }

///它是懒初始化的,在你第一次访问它的时候它才会调用初始化函数进行初始化.但它不是线程安全的,线程安全的使用std::sync::LazyLock;



/// rc是Rust中一个智能指针类型,命名是std::rc::Rc.代表Reference counting.用于在多个地方共享相同数据时,通过引用计数来进行所有权管理.
/// Rc使用引用计数来追踪指向数据的引用数量.当引用计数降为零时,数据会被自动释放
/// Rc允许多个Rc指针共享相同的数据,而无需担心所有权的转移
/// Rc内部存储的数据是不可变的.如果需要可变性,可以使用RefCell或Mutex等内部可变性的机制
/// Rc在处理循环引用时需额外注意,因循环引用会导致引用计数无法降为零,从而导致内存泄漏,为了解决该问题请伤脑筋Weak类型.

pub fn rc_example() {
    use std::rc::Rc;
    let data = Rc::new(32);
    let rc1 = Rc::clone(&data);
    let rc2 = Rc::clone(&data);
    //data的引用 计数为3,当rc1,rc2被丢弃时,引用计数减少
    //Rc允许在多个地方共享不可变数据,通过引用计数来管理所有权.
}

//修改Rc中的数据,使用Cell相关类型配合使用,实现了对不可变类型Rc的数据可变性,是线程不安全的.想线程安全,可以使用Arc.
// pub fn rc_refcell_example() {
//     // #![feature(ref_cell)]
//     use std::cell::{RefCell, RefMut};
//     let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
//     {
//         let mut map: RefMut<_> = shared_map.borrow_mut();
//         map.insert("africa", 92388);
//         map.insert("kyoto", 11837);
//         map.insert("piccadilly", 11826);
//         map.insert("marbles", 38);
//     }
//     let total: i32 = shared_map.borrow().values().sum();
//     println!("{total}");
// }