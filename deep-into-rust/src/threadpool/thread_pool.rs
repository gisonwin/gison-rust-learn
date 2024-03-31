use std::sync::mpsc::channel;
use scoped_thread_pool::*;
use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
/// 线程池是一种并发编程的设计模式，它由一组预先创建的线程组成，用于执行多个任务。主要作用是在任务到达时，重用已创建的线程，
/// 避免频繁地创建和销毁线程，从而提高系统的性能和资源利用率。常用于需要处理大量短期任务或并发请示的应用程序。
/// 线程池的优势
/// - 减少线程创建和销毁的开销：线程的创建和销毁是一项最贵的操作，线程池通过重用线程池减少这些开销，提高了系统的响应速度和效率
/// - 控制并发度：线程池可以限制同时执行的线程数量，从而有效控制系统的并发度，避免资源耗尽和过度竞争。
/// - 任务调度和负载均衡：线程池使用任务队列和调度算法来管理和分配任务，确保任务按照合理的方式分配给可用的线程，实现负载均衡
/// 和最优资源利用。
///
///
///
///
/// Rayon是Rust中一个并行计算库，让你很容易地编写并行代码，以充分利用多核处理器，并提供简单API，允许将迭代操作并行化，从而
/// 加速处理大规模数据集。同时还提供构建线程池的能力。
///
///
/// rayon::ThreadPoolBuilder用于自定义和配置rayon线程池的行为，线程池是其核心部分，它管理并行任务的执行。可以按需求定制
/// 线程池，以便于利用多核处理器的性能优势。

use rayon::*;

pub fn new_thread_pool() {
    //默认情况下rayon会根据cpu内核数量自动设置线程数。
    let pool = ThreadPoolBuilder::new().num_threads(8).thread_name(|i| format!("worker-{}", i))
        .build().unwrap();

    //创建一个全局线程池，且只会初始化一次
    let global_pool = ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();

    //stack_size 用于设置线程栈的大小 start_handler 用于设置线程启动时回调函数。
//     spawn_handler 实现定制化的函数来产生线程。panic_handler提供对panic处理的回调函数。exit_handler提供线程退出时回调。
}


fn fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
    return a + b;
}

pub fn rayon_thread_pool() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let n = pool.install(|| fib(30));
    println!("{}", n);
}

/// threadpool是一个rust库，创建和管理线程池，使并行化任务变得更加容易。
/// 1. 创建线程池:threadpool助你轻松创建线程池,指定线程池大小(同时运行的线程数量).
/// 2.提交任务:创建好了线程池,任务就可以提交给线程池进行执行.任务实现了FnOnce()特质的闭包,通常用于表示您 想要并行执行的工作单元
/// 3.任务调度:线程池会自动将任务分发给可用线程,并在任务完成以后回收线程.
/// 4.等待任务完成: 等线程池中所有任务完成,确保在继续执行后续代码之前,所有任务已完成.
/// 5.错误处理:threadpool提供错误处理机制,方便检测和处理任务执行期间可能发生的错误.
pub fn use_thread_pool() {
    let pool = threadpool::ThreadPool::new(4);
    //create a channel
    let (sender, receiver) = channel();
    //commit a task to thread pool
    for i in 0..8 {
        let s = sender.clone();
        pool.execute(move || {
            let result = i * 2;
            println!("send result=={}", result);
            s.send(result).expect("send fail");
        });
        for _ in 0..8 {
            let result = receiver.recv().expect("receive fail");
            println!("task result == {}", result);
        }
    }
}

pub fn scoped_threadpool() {
    let mut pool = scoped_thread_pool::Pool::new(4);
    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    pool.scoped(|s| {
        for e in &mut vec {
            s.execute(move || {
                *e += 1;
            });
        }
    });
    println!("{:?}", vec);
    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8])
}

pub fn poolite_fibonacci(n:i128) {
    let pool = poolite::Pool::new().unwrap();
    let map = Arc::new(Mutex::new(BTreeMap::<i128, i128>::new()));
    for i in 0..n {
        let map = map.clone();
        pool.push(move || test(i, map));
    }
    pool.join();

    for (k, v) in map.lock().unwrap().iter() {
        println!("k={} \t v={}", k, v);
    }
}

fn test(msg: i128, map: Arc<Mutex<BTreeMap<i128, i128>>>) {
    let res = fibonacci(msg);
    let mut maplock = map.lock().unwrap();
    maplock.insert(msg, res);
}

fn fibonacci(msg: i128) -> i128 {
    match msg {
        0 => 1,
        1 => 1,
        x => fibonacci(x - 1) + fibonacci(x - 2),
    }
}