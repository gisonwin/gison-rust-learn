use std::cell::RefCell;
use std::sync::{Arc, Barrier, Mutex, Once};
use std::{thread, time};
use async_std::prelude::FutureExt;

///同步是多线程程序中一个重要概念,多线程环境下,多个线程可能同时访问某个共享资源,这就导致数据竞争或数据不一致的问题.为了保证数据的安全,需要进行同步操作.
/// 常见的同步需求包括
/// - 互斥:线程在使用共享资源时,同一时刻只允许一个线程访问共享资源,当一个线程使用时,其他线程需要等待,不能同时访问,需要互斥访问.
/// -限制同时访问线程数:对某些共享资源,需要限制同一时刻访问的线程数.
/// - 线程间通信:一个线程需要基于另一个线程的处理结果才能继续执行,需要线程间通信.
/// - 有序访问:对共享资源访问需要按某种顺序进行.
/// 常见同步原语:互斥锁,信号量,条件变量等. 互斥锁可以保证同一时刻只有一个线程可以访问共享资源.信号量可以限制同时访问线程数
/// 条件变量可实现线程间的通信和协议.这些原语的使用可避免同步问题,帮我们正确有效地处理多线程间的同步需求.

/// Arc
///Rust中Arc代表原子引用计数(Atomic Reference Counting),是一种用于多线程环境的智能指针.它允许在多个地方共享数据,同时确保线程的安全性.
/// std::sync::Arc是标准库的一部分,通常情况下Rust中变量是被所有权管理的,但有时我们需要在多个地方共享数据.Arc通过在堆上分配内存,并使用引用
///计数来跟踪数据的所有者数量,确保在不需要的时候正确的释放资源.

fn arc_example() {
    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(46);//可共享的整数
    //创建两个线程,共享对data的引用
    let thread1 = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("thread 1:{}", data);
        })
    };
    let thread2 = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("thread 2:{}", data);
        })
    };
    thread1.join().unwrap();
    thread2.join().unwrap();
}

///Arc(原子引用计数)和Rc(引用计数)都是Rust中用于多所有权的智能指针,它们的区别如下:
/// - 线程安全性:Arc是线程安全的,可安全地在多线程环境中共享,使用原子操作来更新引用计数,确保并发访问时的线程安全性.Rc不是线程安全
/// 的.只适用于单线程环境,因它的引用计数不是原子操作,可能导致在多线程环境中兑态条件和不安全的行为.
/// - 性能开销:Arc使用原子操作来更新引用计数,相对Rc的开销更大,原子操作通常比非原子操作更昂贵.Rc在单线程环境中性能更好,因为它不需要进行原子操作.
/// - 可变性:Arc不能用于可变数据.如需要在多线程环境中共享可变数据,通常会使用Mutex,RwLock等同步原语和Arc.Rc也不能用于可变数据,它无法提供并发访问的安全性.
/// - 引用计数减少时的行为:当Arc的引用计数减少为零时,由于它是原子的,它会正确地释放底层资源(如堆上的数据).Rc在单线程引用计数减少为零时会正确释放资源,但在多线程
/// 环境中可能存在问题,它没有考虑并发情况.
/// 总之,多线程情况下用Arc,单线程情况下使用Rc就好了.
/// 当需要在多线程环境中共享可变数据时,结合使用Arc和Mutex.Mutex互斥锁确保任意时刻只有一个线程能够访问被锁定的数据.
/// 演示使用Arc和Mutex在多线程中共享可变数据
pub fn arc_mutex_example() {
    use std::sync::Arc;
    use std::thread;
    let counter = Arc::new(Mutex::new(0));
    //创建多个线程来增加计数器的值
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //获取锁,确保只有一个线程能够访问计数器.
            //lock方法返回一个MutexGuard,它是一个智能指针,实现了Deref和Drop trait,当MutexGuard
            //被销毁时,会自动释放锁,确保在任何情况下都能正确释放锁.
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
//等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    //打印最终的计数器值
    println!("Final count:{}", *counter.lock().unwrap());
}

/// Arc和RefCell结合使用场景是发生在多线程中需要共享可变状态,但又不需要互斥锁的场景.RefCell允许在运行时进行借用检查,所以在单线程环境下
///使用时,不会像Mutex引入锁的开销.示例演示Arc和RefCell,在多线程环境中共享可变状态.
// pub fn arc_refcell_example() {
//     use std::sync::Arc;
//     use std::cell::RefCell;
//     use std::thread;
//
//     let counter = Arc::new(RefCell::new(0));
//     let mut handles = vec![];
//     for _ in 0..5 {
//         let cnt = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = cnt.borrow_mut();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("Final count:{}", *counter.borrow());
// }

///   Mutex是Rust中互斥锁,解决多线程并发访问共享数据时出现的兑态条件.Mutex提供了一种机制,只有拥有锁的线程才能访问被锁定的数据
/// 其他线程必须等待锁的释放. Lock,try_lock ,poisoning.
/// 为了跨线程支持,一般Mutex与Arc组合使用,这样Mutex对象在每个线程中都能安全访问,lock方法返回实现了Deref trait的MutexGuard对象,所以它会自动解引用,可以
///直接调用被保护对象上的方法,MutexGuard还实现了Drop trait,所以锁会自动解锁,一般你不需要主动调用drop去解锁.


///std::sync::Once用于确保某个操作在整个程序生命周期内只执行一次,主要用于多线程环境中执行初始化代码,确保该代码只被执行一次,即使有多个线程同时调用它.
pub fn sync_once_example() {
    use std::sync::{Once};

    static INIT: Once = Once::new();

    INIT.call_once(|| {
        println!("init once ");
    });

    INIT.call_once(|| {
        print!("init once invoke again");
    });
}

///使用场景:全局初始化:在程序启动时执行一些全局初始化操作,如初始化全局变量,加载配置等,懒加载:在需要时进行一次性初始化,如懒加载全局配置.
///单例模式:通过Once可以实现线程安全的单例模式,确保某个对象在整个程序生命周期内只被初始化一次.

pub fn sync_once_load_config() {
    use std::sync::Once;
    static mut GLOBAL_CONFIG: Option<String> = None;
    static INIT: Once = Once::new();
    fn init_global_config() {
        unsafe {
            GLOBAL_CONFIG = Some("Init global config".to_string());
        }
    }
    fn get_global_config() -> &'static str {
        INIT.call_once(|| init_global_config());
        unsafe {
            GLOBAL_CONFIG.as_ref().unwrap()
        }
    }
    println!("{}", get_global_config());
    println!("{}", get_global_config());
}

/// get_global_config函数通过Once确保init_global_config函数只会被调用一次,从而实现了全局配置的懒加载.
/// OnceCell和OnceLock都是同一族的单次初始化的并发原语,二者区别是:Once是用于确保某个操作在整个程序生命周期内只执行一次的原语,适用于
/// 全局初始化,懒加载和单例模式等场景.OnceCell是一个针对某种数据类型进行包装的懒加载容器,可以在需要时执行一次性初始化,并在之后提供对
/// 初始化值的访问,OnceLock是一个可用于线程安全的懒加载的原语,类似OnceCell,但更简单,只能存储Copy类型的数据,OnceCell可存储任意类型数据.
/// 屏障/栅栏 Barrier
/// Barrier是Rust标准库中一种并发原语,用在多个线程之间创建一个同步点.它允许多个线程在某个点上等待,直到所有线程都到达该点,然后它们可同时继续执行.


pub fn barrier_example() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    let barrier = Arc::new(Barrier::new(3)); //有3个线程参与同步
    let mut handles = vec![]; //创建多个线程
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} working", i);
            thread::sleep(std::time::Duration::from_secs(i as u64));

            barrier.wait();
            println!("Thread {} resumed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 上例中创建了个barrier,并指定了参与同步的线程数量为3.然后创建了3个线程,每个线程模拟一些工作,然后调用barrier.wait()来等待其他线程.
/// 当所有线程都调用了wait后,它们同时继续执行.使用场景-并行计算:当需要确保多个线程在某个点上同步,以便执行某些计算或任务时,可以使用barrier.
/// 迭代步骤同步:一些算法中,可能需要多个步骤,每个步骤的结果都依赖于其他步骤的完成.Barrier可以用于确保所有线程完成当前步骤后再继续下一步.
/// 协同工作阶段:在多阶段的任务中,可使用barrier来同步各个阶段.其灵活性在协调多个线程的执行流程时非常有用.
/// 一旦所有线程都通过wait方法达到同步点后,barrier就被重置,可再次使用,这种重置操作是自动的.barrier内部状态会被重置,下一次调用wait方法时
/// 线程会重新被阻塞,直到所有线程再次到达同步点.这样barrier可被循环使用,用于多轮的同步.

pub fn barrier_reuse_example() {
    use rand::{thread_rng, Rng};
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];
    for _ in 0..10 {
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait1");
            let dur = thread_rng().gen_range(100, 1000);
            thread::sleep(std::time::Duration::from_millis(dur));
            barrier.wait();
            println!("after wait");
            thread::sleep(time::Duration::from_secs(1));

            barrier.wait();

            println!("after wait again");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
