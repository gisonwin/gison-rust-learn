use std::fmt::format;
use std::sync::mpsc;
use std::thread;
use smol::channel::unbounded;

/// Channel是Rust中用于不同线程之间传递消息的机制。主要有以下几个特点：
/// - 通道提供了一种在线程之间安全传递数据的方式.向通道发送数据不会导致竞争条件或死锁.=通道运用了Rust所有权系统来确保消息只被一个接收者获取.
/// 当一个值通过通道发送时,发送者会失去这个值的所有权.
/// - 通道可设置为同步或异步的.同步通道在没有接收者准备好时会阻塞发送者,异步通道则会在后台缓冲未处理的消息.
/// - 通道可以是有边界的或无边界的.有边界意味着通道是一个固定长度的缓冲区,当缓冲区填满时发送会被阻塞.无边界通道则没有限制.
/// - 通道是泛型的.可以传递任何实现了Send,Sync trait的数据.

///通道最适合在不同线程间传递较大数据,或作为线程安全的任务分配机制.对于传递少量数据的情况下,原子类型或Mutex会更高效.通道在Rust中广泛应用于
///多线程,并发场景中,正确使用通道右大大简化多线程编程的复杂性和风险.

/// mpsc.std::sync::mpsc模块用于多生产者,单消费者的通道(multiple producer,single consumer)有以下特点:
/// - mpsc通道只允许有一个接收者.简化了所有权传递,因为每条消息只能被唯一获取一次.
/// - 多个发送者可同时向一个mpsc通道发送消息.通道会自动处理同步并发写访问.
/// - mpsc既支持同步也支持异步,同步通道需要设置边界(缓冲区大小)
/// - 通过mpsc发送的值必须实现Send trait,这确保发送的类型可以安全的在线程间移动
/// - 接收端可通过轮询或等待接收消息,try_recv不会阻塞,recv会阻塞直到有消息可用
/// - mpsc通道在发送端关闭后,接收商会收到一个None消息,表示通道的生命周期结束
/// - mpsc通道通常用来构建线程安全的生产者-消费者模式.多个生产者通过通道发送消息,一个消费者接收处理.吞吐量可达很高水平.


///这个模块提供了基于消息的通道,具体定义了三种类型 Sender,SyncSender,Receiver.Sender,SyncSender向Receiver发送数据,且是可克隆的
///(多生产者),多线程可同时向一个Receiver发送消息.
///异步通道,无限缓冲的通道,channel函数会返回一个(Sender,Receiver)元组,其中所有发送都是异步的永不阻塞的.
///同步通道,有边界的通道,sync_channel函数会返回一个(SyncSender,Receiver)元组,用于挂起消息的存储由一个固定大小的预分配缓冲区组成.
///所有发送都是同步的,会阻塞直到有缓冲区空间可用.如果边界大小设置为0,则会成为约定通道,每个发送方原子地把一条消息传给接收方.
///通过三种类型通道,提供了多生产者单消费者,异步和同步,无限缓冲和有边界缓冲等不同形式的FIFO队列通信机制.

pub fn channel_example() {
    use std::sync::mpsc;
    use std::thread;

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let message = String::from("Hello from the producer!");
        sender.send(message).expect("Failed to send message");
    });

    let received_message = receiver.recv().expect("Failed to receive message");
    println!("Received message:{received_message}");
}

pub fn mpsc_channel_example2() {
    let (sender, receiver) = mpsc::channel();
    for i in 0..3 {
        let tx = sender.clone();
        thread::spawn(move || {
            println!("send {}", i);
            tx.send(i).expect("Failed to send message ");
        });
    }
    for _ in 0..3 {
        let received_message = receiver.recv().expect("Failed to receive message");
        println!("Received message: {received_message}");
    }
}

pub fn mpsc_sync_channel_example2() {
    let (sender, receiver) = mpsc::sync_channel(3);
    for i in 0..3 {
        let sender = sender.clone();
        thread::spawn(move || sender.send(format!("{}-{}", "ok", i)).unwrap()
        );
    }
    drop(sender);
    while let Ok(msg) = receiver.recv() {
        println!("{msg}");
    }
    println!("mpsc_sync_channel_example2 completed");
}

pub fn mpsc_sync_channel_with_zero() {
    let (sender, receiver) = mpsc::sync_channel::<String>(0);
    //producer thread
    thread::spawn(move || {
        for i in 0..5 {
            let msg_string = format!("{}-{}", "producer", i);
            sender.send(msg_string.clone()).expect("Failed to send message");
            println!("Send message:{msg_string}");
        }
    });
    //consumer thread
    thread::spawn(move || {
        for i in 0..5 {
            let received_message = receiver.recv().expect("Failed to receive a message");
            println!("Received message: {received_message}");
        }
    });
    //wait for all thread complete
    thread::sleep(std::time::Duration::from_secs(10));
}

///下面是一些知名通道库,crossbeam-channel,flume,tokio,crossfire等,可以满足不同的需求.
/// crossbeam-channel,提供了多生产者多消费者的线程安全通道.主要功能和特点:
/// - 提供unbounded和bounded两种通道.unbounded通道可以无限制地发送消息,bounded通道可以设置容量上限.
/// - 支持多生产者多消费者,多个线程可同时发送或接收消息.
/// - 提供select!宏,可同时在多个通道上进行操作.类似Go lang channel的便利.提供了Receiver,Sender等抽象,Usage风格友好.

pub fn crossbeam_channel_bounded() {
    use crossbeam_channel::{bounded, Sender, Receiver};
    use std::thread;
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = bounded(10);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            sender.send(i).unwrap();
            println!("Send: {i}");
        }
    });

    let consumer = thread::spawn(move || {
        for i in 0..10 {
            let data = receiver.recv().unwrap();
            println!("Received:{data}");
        }
    });
    //创建了一个有界通道为10,然后启动了一个生产者线程,向通道发送0到9数字,同时启动了一个消费者线程,从通道接收数据并打印出来,最后等待两个线程完成.
    producer.join().unwrap();
    consumer.join().unwrap();
}

///理论上生产者线程不断发送递增的数字到无界通道,而消费者线程只接收前10个数字并打印出来.通道由于是无界的,生产者线程可以一直发送数据,但在实际运行中会发生
/// called `Result::unwrap()` on an `Err` value: "SendError(..)"
pub fn crossbeam_channel_unbounded() {
    use crossbeam_channel::{unbounded, Sender, Receiver};
    use std::thread;
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = unbounded();

    let producer = thread::spawn(move || {
        for i in 0.. {
            sender.send(i).expect("Failed to send message");
            println!("Producer: {i}");
        }
    });
    let consumer = thread::spawn(move || {
        for i in 0..15 {
            let data = receiver.recv().unwrap();
            println!("Consumer-- {data}");
        }
    });
    producer.join().unwrap();
    consumer.join().unwrap();
}

///select! macro是crossbeam-channel提供的一种用于监听多个通道的事件并执行相应的操作的方式.对于多路复用情况下非常有用,可根据不同通道事件执行不同的逻辑.
/// 演示使用select!监听两个通道,并根据事件执行相应的操作.
pub fn crossbeam_select_macro() {
    use crossbeam_channel::{unbounded, select, Sender, Receiver};
    use std::thread;
//create two channels
    let (sender1, receiver1): (Sender<&str>, Receiver<&str>) = unbounded();
    let (sender2, receiver2): (Sender<&str>, Receiver<&str>) = unbounded();
    //create a producer ,send message to channel one
    let producer1 = thread::spawn(move || {
        for i in 0..5 {
            sender1.send(&format!("Channel 1:Message {}", i)).unwrap();
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });
    //create second producer send message to channel two
    let producer2 = thread::spawn(move || {
        for i in 0..5 {
            sender2.send(&format!("Channel 2:Message:{}", i)).unwrap();
            thread::sleep(std::time::Duration::from_millis(300));
        }
    });
    let consumer = thread::spawn(move || {
        for i in 0..10 {
            select! {
                recv(receiver1) -> msg1 =>{
                    match msg1 {
                        Ok(msg) => println!("Received from channel 1:{}",msg),
                        Err(_) => println!("Channel 1 closed"),
                    }
                }
                recv(receiver2) -> msg2 =>{
                    match msg2 {
                        Ok(msg) => println!("Received from Channel 2:{}",msg),
                        Err(_) => println!("Channel 2 closed"),
                    }
                }
            }
        }
    });

    producer1.join().unwrap();
    producer2.join().unwrap();
    consumer.join().unwrap();
}


