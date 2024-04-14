use std::sync::mpsc;
use std::thread;

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




