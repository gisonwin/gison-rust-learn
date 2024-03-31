use std::cell::RefCell;
use std::thread;


//以下代码是无法编译的，因为线程外的a没有办法move到两个thread中，即使move到一个thread中
// 外部的线程也没有办法再使用它了。为了解决该问题，我们使用scoped thread.
// pub fn wrong_start_thread_without_scoped() {
//     let mut a = vec![1, 2, 3];
//     let mut x = 0;
//
//     thread::spawn(move || {
//         println!("hello from the first scoped thread");
//         dbg!(&a);
//     });
//
//     thread::spawn(move || {
//         println!("hello from the second scoped thread");
//         x += a[0] + a[2];
//     });
//
//     println!("hello from the main thread");
//
//     a.push(4);
//     assert_eq!(x, a.len())
// }

pub fn start_scoped_thread() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    thread::scope(|s| {
        s.spawn(|| {
            print!("hello from the first scoped thread");
            dbg!(&a);
        });
        s.spawn(|| {
            print!("hello from the second scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });
    a.push(4);
    assert_eq!(x, a.len());
}
//调用thread::scope函数，使用s参数启动了两个scoped thread，它们使用了外部的变量a x,因为我们对a只是读，对x只有单线程
// 的写，所以不用考虑并发问题。thread::scope 返回 后，两个线程已经执行完毕，所以外部线程又可以访问变量了。
// 标准库的scope功能没有进一步扩展，事实上，在新的scoped thread我们是不是还可以启动新的scope thread。这样就实现了
// java一样的Fork-join父子线程。


// ThreadLocal

// ThreadLocal为rust程序提供了thread-local storage实现，TLS(thread-local storage)可以存储数据到全局变量中，每个线程
// 都有这个存储变量的副本，线程不会分享这个数据，副本是线程独有的，所以对其访问不需要同步控制。thread-local key拥有它的值，且在线程
// 退出时此值会被销毁。thread_local!宏创建thread-local key,它可以包含'static的值。它使用with访问函数去访问值。
// 如果修改值，需要结合Cell和RefCell，可理解它们为不可变变量提供内部可修改性。例子如下：
pub fn start_threads_with_threadlocal() {
    thread_local! {static COUNTER:RefCell<u32> = RefCell::new(1)};

    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    let handle = thread::spawn(move || {
        COUNTER.with(|c| { *c.borrow_mut() = 3; });
        COUNTER.with(|c| {
            println!("hello from a thread7,c={}", *c.borrow());
        });
    });

    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| { *c.borrow_mut() = 4; });
        COUNTER.with(|c| {
            println!("hello from a thread8,c={}", *c.borrow());
        });
    });

    handle.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|c| {
        println!("hello from main thread,c={}", *c.borrow());
    });
}


// move在thread中使用
// 使不使用move依赖相应的闭包是否要获取外部变量的所有权。如果不获取外部变量的所有权，则不使用move，
// 大部分情况下我们会使用外部变量，所以常用move。
pub fn start_one_thread_with_move(){
    let x = 100;

    let handle = thread::spawn(move || {
        println!("with move,x={}", x);
    });
    handle.join().unwrap();
//move 上面把x的所有权交给了第一个子线程，为什么第二个依然可以move并使用呢，这是因为x是i32类型的，该类型实现了
//     Copy trait,实际move的时候是复制它的值，所以第二个子线程依然可以使用x的值。
//     如果我们将x的类型替换为一个未实现Copy的类型，该代码编译就无法通过了。因为x的所有权已经转移到第一个子线程了。
    let handle = thread::spawn(move || {
        println!("with move,x={}", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(|| { println!("without move") });
    handle.join().unwrap();
}

pub fn start_one_thread_with_move2(){
    let x = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("with move,x={:?}", x);
    });
    handle.join().unwrap();
    // let handle = thread::spawn(move || {
    //     println!("with move,x={:?}", x);
    // });
    //
    // handle.join().unwrap();

    let handle = thread::spawn(|| { println!("without move") });
    handle.join().unwrap();
}


// affinity 将线程绑定在一个核上或几个核上，如果绑定多核，使用crate affinity
#[cfg(not(target_os="macos"))]
pub fn use_affinity(){
    //select every second core
    let cores:Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();
    println!("binding thread to cores:{:?}",&cores);

    affinity::set_thread_affinity(&cores).unwrap();
    println!("Current thread affinity :{:?}",affinity::get_thread_affinity().unwrap());

}