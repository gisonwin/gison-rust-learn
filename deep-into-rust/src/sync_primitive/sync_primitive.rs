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
