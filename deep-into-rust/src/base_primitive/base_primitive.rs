// #![feature(exclusive_wrapper)]
// use std::sync::Exclusive;
use std::cell::RefCell;
use std::sync::{Arc, Barrier, Condvar, Mutex, Once};
use std::{thread, time};
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{channel, sync_channel};
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

/// 条件变量condvar
/// Condvar是Rust标准库中条件变量(Condition Variable),用于多线程间进行线程间协调和通信。条件变量允许线程等待某个特定的条件成立，当条件满足时，线程可以被唤醒并继续执行.
/// 下例创建了一个Mutex和Condvar，其中Mutex用于保护共享状态(条件)，而Condvar用于等待和唤醒线程。多个线程在Mutex加锁后，通过condvar.wait()方法等待条件满足，然后在主线程中修改
/// 条件，并通过condvar.notify_all()唤醒所有等待的线程。
///
pub fn sync_condvar_example() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    let mutex = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());

    let mut handles = vec![];
    for i in 0..3 {
        let mutex = Arc::clone(&mutex);
        let condvar = Arc::clone(&condvar);
        let handle = thread::spawn(move || {
            let mut guard = mutex.lock().unwrap();//获取mutex 锁
            while !*guard {
                guard = condvar.wait(guard).unwrap();
            }
            println!("Thread{} woke up", i)
        });
        handles.push(handle);
    }
    //模拟条件满足后唤醒等待的线程
    thread::sleep(std::time::Duration::from_secs(2));

    //修改条件，并唤醒等待的线程
    {
        let mut guard = mutex.lock().unwrap();
        *guard = true;
        condvar.notify_all();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 使用场景:
/// - 线程间同步:Condvar可用于线程之间同步,使得线程能够等待某个条件的成立而不是轮询检查.
/// - 生产者-消费者模型:多线程环境中,生产者线程可通过条件变量通知消费者线程有新的数据产生
/// - 线程池:线程池中,任务线程可以等待条件变量,等待新的任务到达时被唤醒执行.
/// Condvar一般配合mutex使用,确保在等待和修改条件时的线程安全性.通过调用notify_one()发出信号,信号发出后,Condvar随机选一个正在等待信号的线程,并释放该线程.
/// 通过调用notify_all()发出信号,信号发出后Condvar会释放所有正在等待信号的线程.



///  LazyCell && LazyLock
/// 二者均用于懒惰初始化对象的工具,LazyCell用于懒惰初始化值,LazyLock用于懒惰初始化资源.
///---------------------------------------------------------------------------
///类型            用途             初始化时机                   线程安全
///---------------------------------------------------------------------------
///LazyCell    懒惰初始化值           第一次访问                        否
///LazyLock    懒惰初始化资源          第一次获取锁                      是
///OnceCell    懒惰初始化单例值         第一次调用                     get_or_init()方法
///OnceLock    懒惰初始化互斥锁         第一次调用                     lock()方法
///---------------------------------------------------------------------------


//           Exclusive
// Rust中Exclusive是用于保证某个资源只被一个线程访问的工具,std::sync::Exclusive.
// Exclusive 仅提供对底层值的可变访问,也称被底层值的独占访问,它不提供对底层值的不可变或共享访问.
// 虽然看起来不太有用,但它允许Exclusive无条件实现Sync.sync的安全要求是,对于 Exclusive而言,它必须可安全地跨线程共享,就是说
// &Exclusive跨越线程时必须是安全的.
// pub fn sync_exclusive_example() {
//     use std::sync::Exclusive;
//     let mut exclusive = Exclusive::new(92);
//     println!("ready");
//     std::thread::spawn(move || {
//         let counter = exclusive.get_mut();
//         println!("{}", *counter);
//         *counter = 100;
//     }).join().unwrap();
// }

/// mpsc
/// mpsc是标准库中一个模块，提供了多生产者，单消费者(Multiple Producers,Single Consumer)的消息传递通道。mpsc是multiple-producers,single-consumer的缩写。
/// 该模块基于channel传递消息，具体定义了三具类型： Sender,SyncSender,Receiver
/// Sender:发送者，用于异步发送消息到Receiver，可clone
/// SyncSender：同步发送者，用于同步发送消息到Receiver,可clone
/// Receiver：接收者，用于从异步channel或同步channel中接收令牌，只能有一个线程访问。
/// 通道有两种类型:异步的，无限缓冲区的通道。channel函数返回一个(Sender，Receiver)元组，其中所有发送将是异步的(永不阻塞)。该通道在概念上具有无限的缓冲区。
/// 同步的，有办的通道，sync_channel函数返回一个(SyncSender,Receiver)元组，等发送消息的存储区是一个固定大小的预分配缓冲区。所有发送都是同步的，通过阻塞直到有空闲的缓冲区空间。如果设置大小为0也是允许的，
/// 这将使通道变成一个“约定”通道，每个发送方原子地将一条消息交给接收方。
/// 使用场景:并发消息传递：适用于多个线程(生产者)向一个线程(消费者)发送消息的场景。任务协调：用于协调多个并发任务的执行流程。
/// rust的mpsc和go的channel类似，使用起来比较简单
pub fn simple_channel_example() {
    use std::thread;
    use std::sync::mpsc::channel;
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 10);
}

pub fn mpsc_channel_example() {
    //create a shared channel that can be sent along from many threads where tx is the sending half(tx for transmission),
    //rs is the receiving half(rs for receiving).
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    for i in 0..10 {
        let j = rx.recv().unwrap();
        assert!(j >= 0 && j < 10);
    }
}

pub fn mpsc_sync_channel_example() {
    use std::sync::mpsc::sync_channel;
    let (tx, rx) = sync_channel(3);
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
            println!("send {}", i);
        });
    }
    //drop the last sender to stop rx waiting for message.the program will not complete if we comment this out.
    // ALL 'tx' needs to be dropped for 'rx' to have 'Err'.
    drop(tx);
    while let Ok(msg) = rx.recv() {
        println!("receive {msg}");
    }
    println!("completed");
}

/// Rust标准库中，没有提供原生MPMC(Multiple Producers,Multiple Consumers)通道。std::sync::mpsc模块提供的是单一消费通道，主要出于设计和性能考虑。
/// mpsc通道实现相对简单,更容易满足特定的性能需求,并且在很多情况下是足够的.同时其场景最常见,如在线程池中有一个任务队列,多个生产者将任务推送队列中,而单个消费者负责执行这些任务.


/// 信号量 Semaphore
///  原子操作 atomic
/// Rust中原子操作(Atomic Operation)是一种特殊操作,可在多线程环境中以原子方式进行,即不会被其他线程操作打断.原子操作可保证数据的线程安全性,避免数据竞争.std::sync::atomic模块提供一系列用于
///原子操作的类型和函数.atomic适用场景:- 保证某个值的一致性.- 防止多个线程同时修改某个值.-实现互斥锁.
/// Rust原子类型遵循与C++20 atomic相同的规则.atomic_ref.创建Rust原子类型的一个引用,相当于C++中创建一个atomic_ref.当共享引用的生命周期结束时,atomic_ref也会被销毁.
/// AtomicBool,AtomicIsize,AtomicUsize,AtomicI8,AtomicU16. 每个方法都带有一个Ordering参数,表示该操作的内存屏障的强度.原子变量在线程间安全共享(实现了Sync),但它本身不提供共享机制,遵循Rust的线程模型.
/// 共享一个原子变量常见方式是把它放在一个Arc中(一个原子引用计数的共享指针).原子类型可存储在静态变量中,像AtomicBool::new这样常量化初始化器初始化.原子静态变量通常用于懒全局初始化.
/// AtomicI64和I64之间转换,以及原子操作load,store,swap,compare_and_swap (CAS),compare_exchange,fetch_add,fetch_sub,fetch_and,fetch_nand,fetch_or,fetch_xor,fetch_max,fetch_min.
/// -store 原子写入 -load 原子读取 -swap 原子交换 -compare_and_swap 原子比较并交换 -fetch_add:原子加法后返回旧值.
/// Rust中Ordering枚举用于指定原子操作时的内存屏障(memory ordering).
/// - Ordering::Relaxed:Rust中最轻量级的内存屏障,没有对执行顺序进行强制排序.允许编译器和处理器在原子操作周围进行指令重排;C++具有相似语义,允许编译器和处理器在原子操作周围进行轻量级指令重排.
/// - Ordering::Acquire:Rust插入一个获取内存屏障,防止后续的读操作被重排序到当前操作之前.确保当前操作之前的所有读取操作都在当前操作之前执行.C++中memory_order_acquire表示获取操作,确保当前操作之前的读取操作都在当前操作之前执行.
/// - Ordering::Release:Rust插入一个内存屏障,防止之前写操作被重排序到当前操作之后.确保当前操作之后的所有写操作都在当前操作之后执行.C++中memory_order_release表示释放操作,确保之前的写操作都在当前操作之后执行.
/// - Ordering::AcqRel:Rust插入一个获取释放内存屏障,既确保当前操作之前的所有读操作都在当前操作之前执行,又确保之前的所有写操作都在当前操作之后执行.它提供一种平衡,适用于某些获取和释放操作交替进行的场景.C++也是表示获取释放操作
///  ,它是获取和释放的组合.确保当前操作之前的所有读取操作都在当前操作之前执行,同时确保之前的所有写操作都在当前操作之后执行.
/// - Ordering:SeqCst:Rust插入一个全序内存屏障,保证所有线程都能看到一致的操作顺序.是最强内存顺序,用于实现全局同步.C++中,memory_order_seq_cst也表示全序操作,保证所有线程都能看到一致的操作顺序.也是C++中最强内存顺序.
/// 合理选择Ordering可最大程度提高性能,同时保证需要的内存序约束.使用原子操作时需要小心选择合适的Ordering,避免竞态条件和数据竞争.

///Ordering::Relaxed是最轻量级的内存顺序,允许编译器和处理器在原子操作周围进行指令重排,不提供强制的执行排序.这样以获取更高的性能.
pub fn test(){}




// pub fn ordering_relaxed_example() {
//     use std::sync::atomic::{AtomicBool, Ordering};
//
//     let atomic_bool = AtomicBool::new(false);
//
//
//     let producer_thread = thread::scope(|s| {
//         s.spawn(move || {
//             atomic_bool.store(true, Ordering::Relaxed);
//         });
//         s.spawn(move || {
//             let value = atomic_bool.load(Ordering::Relaxed);
//             println!("Received value: {}", value);
//         })
//     });
//     // let consumer_thread = thread::spawn(move || {
//     //     let value = atomic_bool.load(Ordering::Relaxed);
//     //     println!("Received value: {}", value);
//     // });
//     producer_thread.join().unwrap();
//     // consumer_thread.join().unwrap();
// }