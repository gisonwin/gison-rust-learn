use std::sync::{Arc, Mutex};
use std::thread;
use async_std::prelude::FutureExt;
use futures::TryFutureExt;
use tokio::io::AsyncSeek;

///Rust中提供一些集合类型 Vec<T>,HashMap<K,V>,HashSet<T>,VecDeque<T>,LinkedList<T>,BTreeMap<K,V>,BTreeSet<T>等。
/// Vec:可变大小的数组,允许在头部或尾部高效地添加和删除元素,类似于C++的vector,java's ArrayList
/// HashMap<K,V>:哈希映射,允许通过key快速查找值,
/// HashSet:基于Hash的集合,可快速判断一个值是否在集合中,
///VecDeque:双端队列,允许在头部或尾部高效地添加和删除元素.
///LinkedList:链表数据结构,允许在头部或尾部快速添加和删除元素
///BTreeMap,有序的映射,可通过key快速查找,同时保持元素的排序.使用B树作为底层数据结构
///BTreeSet:有序的集合,元素会自动排序,使用BTree作为底层数据结构.
///上述类型都是线程不安全的,没有办法在线程中共享使用.我们可使用前面介绍的并发原语,对其进行包装,使之成为线程安全的.

/// 线程安全的Vec
/// 使用Arc(原子引用计数)和Mutex(互斥锁)组合实现安全的Vec,Arc允许多个线程共享拥有相同数据的所有权,而Mutex用在访问数据时进行同步,确保只有一个线程能够修改数据.

pub fn arc_mutex_vec_example() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    let shared_vec = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for i in 0..5 {
        let shared_vec = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            let mut vec = shared_vec.lock().unwrap();
            println!("{}", i);
            vec.push(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let final_vec = shared_vec.lock().unwrap();
    println!("Final vec:{:?}", *final_vec);
}

/// 实现安全hashmap,使用Arc和Mutex组合,或使用RwLock来提供更细粒度的并发控制.

pub fn arc_mutex_hashmap_example() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;

    let shared_map = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    for i in 0..5 {
        let shared_map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = shared_map.lock().unwrap();
            // i += 1;
            println!("{}", i);
            map.insert(i, i * i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_map = shared_map.lock().unwrap();
    println!("final map:{:?}", final_map);
}

/// 套路一样,使用Arc<Mutex<T>>实现,这是一种常见的实现线程安全的集合类型,但不是唯一选择,其基本思想是使用Arc原子引用计数来实现多线程间的所有权共享,而Mutex提供互斥锁,确保在任何时刻只有一个线程能够修改数据.
/// 有的场景使用Arc<RwLock<T>>更合适,允许多个线程同时读取数据,但只有一个线程能够写入数据,适用用频繁读取和写操作少的场景.

/// DashMap是极快的Rust并发map实现,关联array/hashmap实现.它试图实现一个类似于std::collections::HashMap的简易API,并能处理并发.它可以替换RwLock<HashMap<K,V>>.实现就是将DashMap放入Arc中,并在线程
/// 之间共享它,同时仍能修改它.

pub fn arc_dashmap_example() {
    use dashmap::DashMap;
    let dash_map = Arc::new(DashMap::new());
    let mut handles = vec![];
    for i in 0..10 {
        let map = Arc::clone(&dash_map);
        handles.push(thread::spawn(move || { map.insert(i, i * i) }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("final dash map:{:?}", dash_map);
}

/// evmap(eventual map),基于事件的并发map.允许多个线程并发地读取和写入map,同时支持观察者模式,允许在map的变化上注册事件监听器.
/// - 并发读写:evmap允许多个线程并发读取和写入映射,不需要使用锁.通过将map分为多个片段来实现的,每个片段都可以独立的读取和写入.
/// - 事件触发:evmap允许在map的变化上注册监听器.当map发生变化时,注册的监听器就会被触发,从而允许用户执行自定义的逻辑.
/// - 键和值:key,value可是任意类型,只要实现了Clone,Eq trait即可.允许用户使用自定义类型作为key,value
/// - 异步触发事件:支持异步事件触发,让在事件发生时执行一些异步任务成为现实.


pub fn ev_map_example() {
    let (mut book_reviews_w, book_reviews_r) = evmap::new();
    let w = Arc::new(Mutex::new(book_reviews_w));
    let writes: Vec<_> = (0..4).map(|i| {
        let w = w.clone();
        thread::spawn(move || {
            let mut w = w.lock().unwrap();
            w.insert(i, true);
            w.publish();
        })
    }).collect();
    while book_reviews_r.len() < 4 {
        thread::yield_now();
    }
    for w in writes.into_iter() {
        assert!(w.join().is_ok());
    }
}
/// arc-swap
/// 提供了一个基于Arc和Atomic的数据结构,用在多线程间原子地交换数据.目的是提供一种高效的方式来实现线程间共享数据的更新,避免锁的开销.Atomic<Arc<T>>,RwLock<Arc<T>>.
///场景:一些数据结构经常被读取而很少被更新,如服务的配置项,路由表,每几分钟更新一次的某些数据快照等.
/// - 快速,频繁并从多个线程并发读取数据结构的当前值.
/// - 在更长时间内使用数据结构相同的版本,查询应该由数据的一致版本回答,数据包应该由旧版本或新版本的路由表路由,而不是组合路由
/// - 在不中断的情况下执行更新.

///RwLock<T>在整个处理时间内保持读锁,但更新会暂停所有处理直到完成.更好的选择是RwLock<Arc<T>>,然后获得锁,cloneArc并解锁.但这会受到DPU级别的争用(锁和Arc的引用计数)影响,相对比较慢.根据实现
///的不同,稳定的reader流入可能会阻塞更新任意长的时间.此时就可以ArcSwap来替代,并解决了上述问题.

pub fn arc_swap_examples(){
    use arc_swap::ArcSwap;
    let data = ArcSwap::new(1.into());
    println!("Initial Value:{}", data.load());
    data.store(Arc::new(2));
    println!("New Value:{}", data.load());

}