use std::{thread, time::Duration};

pub fn start_one_thread() {
    let handle = thread::spawn(|| { println!("hello from a thread!") });
    handle.join().unwrap();
}

pub fn start_one_thread_result() {
    let handle = thread::spawn(|| {
        print!("Hello from a thread!");
        200
    });
    match handle.join() {
        Ok(v) => { print!("thread result:{}", v); }
        Err(e) => { println!("error :{:?}", e) }
    }
}

pub fn start_two_threads() {
    let handle
        = thread::spawn(|| { println!("Thread1") });
    let handle2
        = thread::spawn(|| { println!("Thread2") });
    handle.join().unwrap();
    handle2.join().unwrap();
}

// start n thread,use a vector save the thread's handle
pub fn start_n_thread() {
    const N: isize = 10;
    let handles: Vec<_> = (0..N).map(|i| {
        thread::spawn(move || { println!("Thread {}", i + 1); })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn start_one_thread_builder() {
    let thread_current = thread::current();
    println!("current thread: {:?},{:?}", thread_current.id(), thread_current.name());
    //set stack size 32*1024
    let builder = thread::Builder::new().name("learn thread".into()).stack_size(32 * 1024);
    let handle
        = builder.spawn(|| {
        let current_thread = thread::current();

        println!("child thread: {:?} ,{:?}", current_thread.id(), current_thread.name())
    }).unwrap();
    handle.join().unwrap();
}

pub fn available_cpu() {
    let count = thread::available_parallelism().unwrap().get();
    print!("current computer has {} cpu(s)", count);
    // let amount
    //     =  thread_amount::thread_amount();
    // if !amount.is_none {
    //     println!("thread_amount:{}",amount);
    // }
}

// sleep保证当前线程指定的时间，会阻塞当前的线程， 所以不要在异步的代码中调用它。
// 如果时间设置为0，不同平台处理不一样，Unix类平台会立即返回，不用调用nanosleep系统调用，
// windows平台总是会调用底层的sleep系统调用。如果只是想让渡出时间片，不用设置时间为0，调用yield_now函数即可。
pub fn start_thread_with_sleep() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        println!("thread sleep 2000");
    });
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("thread sleep 1000");
    });
    handle.join().unwrap();
    handle1.join().unwrap();
}

pub fn start_thread_with_yield() {
    let handle = thread::spawn(|| {
        thread::yield_now();
        println!("yield now");
    });
    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield in another thread ");
    });
    handle.join().unwrap();
    handle2.join().unwrap();
}
//休眠时间不确定时，如果想让某个线程休眠，将来在某个事件之后，再主动唤醒它，就可以使用park unpark方法。
// 我们认为每个线程都有一个令牌(token),最初该令牌不存在：
// - thread::park 将阻塞当前线程，直到线程的令牌可用。 此时它以原子操作使用令牌。thread::park_timeout执行相同的操作，但允许指定阻止线程的最长时间，和sleep不同，它可以还未到超时的时候就被唤醒。
// - thread::unpark 方法以原子方式使该令牌可用。由于令牌初始不存在，unpark会导致紧接着的park调用立即返回

pub fn thread_park2() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        println!("a park thread in case of unpark first");
    });
    handle.thread().unpark();
    //如果调用unpark 接下来的park会立即 返回
    handle.join().unwrap();
}


fn main() {
    // start_one_thread();
    // start_one_thread_result();
    // start_two_threads();
    // start_n_thread();
    // start_one_thread_builder();
    // available_cpu();
    // start_thread_with_sleep();
    // start_thread_with_yield();
    thread_park2();
}

