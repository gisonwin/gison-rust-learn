use tokio::join;
use std::thread;
use rayon::ThreadPool;
use tokio::sync::mpsc;

///异步编程是一种并发编程,通过在任务执行期间不阻塞线程方式,提高系统的并发能力和响应性.异步编程可以更好地处理IO密集型任务
///和并发请示,提高系统的吞吐量和性能.
/// 异步编程有以下优势:
/// - 提高系统的并发能力和响应速度
/// - 减少线程等待时间,提高资源利用率
/// - 可以处理大量并发请示或任务
/// - 支持高效的事件驱动编程风格

///异步编程应用场景:
/// - 网络编程:处理大量的并发网络请示
/// - IO密集型任务:如文本操作、数据库访问
/// -用户界面和图形渲染：保持用户界面的流畅响应
/// - 并行计算:加速复杂计算任务的执行


///Rust中异步编程目标是实现高性能、无安全漏洞的异步应用程序，同时提供简洁的语法和丰富的异步库。
/// - OS线程:适合作为语言的原生并发模型,但线程间的同步很困难,且线程间的上下文切换损耗较大.使用线程池在一定程度上可提升性能,但
/// 是对于IO密集应用场景来说,线程池还是不够看.
/// - 事件驱动(Event Driven)模型:经常跟回调(Callback)一起使用,最大问题是存在回调地釞风险:非线性控制流和结果处理导致了数据
/// 流向和错误传播变得难以掌控,且导致代码可维护性和可读性大幅降低,JS就存在回调地狱.
/// - 协程(Coroutines):Go语言的杀手锏之一.协程跟线程类似,无需改变编程模型,同时还支持大量任务并发运行.但协程抽象层次过高,
/// 导致用户无法接触到底层的细节,对于系统编程语言来自定义异步运行时是难以接受的.
/// -actor模型:是Erlang杀手锏之一,它将所有并发计算分割成一个一个单元,这些单元被称为actor,单元之间通过消息传递的方式进行通信和
/// 数据传递,与分布式系统的设计理念很相近.actor模型跟现实很贴近,相对来说很容易实现,但是一旦遇到流控制、失败重试等场景时会不好用。
/// - async/await模型:该模型性能高还支持底层编程，同时又像线程和协程一样无需改变编程模型，但async模型的内部实现机制过于复杂，
/// 对于用户来说,理解和使用起来不如线程和协程简单,但是rust已帮我们封闭好,我们只管使用即可.

///Rust选择了同时提供多线程编程和async编程:
/// - 前者通过标准库实现,当你无需很高并发时,如需要并行计算时,可以选择它,优点是线程内的代码执行效率更高,实现更直观更简单.
/// - 后者通过语言特性+标准库+三方库实现,当你需要高并发、异步IO时就选择它。


///异步运行时是Rust中支持异步编程的运行时环境,负责管理异步任务的执行和调度.提供了任务队列,线程池和事件循环等基础设施,支持异步
///任务的并发执行和事件驱动的编程模型.Rust没有内置异步调用所必须的运行时,主要的Rust异步运行时包括:
/// - Tokio:Rust异步运行时首选,拥有强大的性能和生态系统,提供了异步TCP/UDP套接字,线程池,定时器等功能
/// - async-std:与Tokio类似的异步抽象,代码较简洁,易上手
/// - smol 一个轻量级的运行时,侧重于simplicity,ergonomics,small 简单性,易用性,小巧.
/// - futures/futures-lite
/// - bytedance/monoio


///异步编程模型包含一些关键组件和概念:
/// - 异步函数和异步块:使用async关键字定义的异步函数和异步代码块
///   async fn foo() ->u8 {5}
///   fn bar() -> impl Future<Output=u8>{
///   async {
///     let x:u8 =foo().await();
///     x+5
///    }
/// }


/// async 语句块和async fn区别是前者无法显式声明返回值.如下
/// async fn foo() -> Result<u8,String> {
///     Ok(1)
/// }
///
/// async fn bar() -> Result<u8,String>{
///     Ok(1)
/// }
/// pub fn main(){
///     let fut = async{
///         foo().await?;
///         bar().await?;
///         Ok(())
///     };
/// }
/// 编译器无法推断出Result<T,E>中E的类型,且编译器的提示consider giving fut a type,也不对,目前还没有办法为async语句块指定返回.
///  既然编译器无法推断出类型,咱们就给它更多提示,可以使用::<...>方式来增加类型注释 :
/// let fut = async{
///     foo().await?;
///     bar().await?;
///     Ok::<(),String>(())
/// }
/// await关键字,在异步函数内部使用await关键字等待异步操作完成
/// async/await是Rust语法的一部分,它在遇到阻塞操作时如IO会让出当前线程的所有权而不是阻塞当前线程,这样就允许当前线程继续去执行
/// 其他代码,最终实现并发. async直到被执行器pool或await后才会开始运行,而await是最常用的运行future的方法,当await被调用时,
/// 它会尝试运行future直到完成,但当该future阻塞时,就会让出当前线程的控制权.当future后面准备再一次运行时(如从socket中读取到了数据)
/// 执行器会得到通知,并再次运行该future,如果循环直到完成.
/// future trait表示异步任务的future trait,提供异步任务的执行和状态管理.
///  pub trait Future {
///     type Output;
///     //required method
///     fn pool(self:Pin<&mut Self>,cx:&mut Context<'_>) -> Pool<Self::Output>;
/// }


///  async用于定义异步函数,表示函数体中包含异步代码,await用于等待异步操作完成,并返回异步操作的结果.
///  async函数返回实现了Future trait的类型.异步函数可在其他异步函数中使用await关键字等待异步操作完成,调用异步函数时,会返回
/// 一个实现了future trait的对象,可以通过调用await方法等待结果.
/// 异步块是一种在异步函数内部创建的临时异步上下文,可以使用async关键字创建.异步闭包是一种将异步代码封装在闭包中的方式.
/// 异步块和异步闭包允许在同步上下文中使用await等待异步操作完成.
///
///
/// Future trait表示一个异步任务,提供异步任务的执行和状态管理.
///
fn get_two_sites() {
    let thread_one = thread::spawn(|| download("https://course.rs"));
    let thread_two = thread::spawn(|| download("https://fancy.rs"));
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

fn download(p0: &str) -> bool {
    true
}

/// 以上的函数在小项目中去用线程下载文件没有问题,当下载文件的并发请求多起来，一个下载任务占用一个线程的模式就太重了，很容易成为程序的瓶颈。
/// 我们可以用async方式来解决
async fn get_two_sites_async() {
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");
    //同时运行两个future,直至完成
    let x = join!(future_one,future_two).await;
    println!("{}", x);
}

fn download_async(p0: &str) {
    todo!()
}

/// 上述代码在异步运行时使用一定数量的线程来调试代码的运行.

/// Tokio是Rust异步编程最重要的运行时库,提供了异步IO,异步任务调度,同步原语等功能.主要组件包括:
/// - tokio:核心运行时,提供任务调度,io资源等
/// - tokio::net 异步TCP,UDP的实现
/// - tokio::sync 互斥量,信号量等并发原语.
/// - tokio::time 时间相关工具
/// - tokio::fs 异步文件IO
#[tokio::main]
async fn main() {
    //执行异步任务
    let join_handle = tokio::spawn(async {
        //do work
    });
    //等待任务完成
    join_handle.await.expect("panic message");
}

/// main函数前必须加async,且加上#[tokio::main]属性,这样该main函数就会在异步运行时运行.下面是显式创建运行时方式
pub fn tokio_async() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from tokio");
        rt.spawn(async {
            println!("Hello from a tokio task");
            println!(" in spawn");
        }).await.unwrap();
    }
    );
    rt.spawn_blocking(|| println!("in spawn_blocking"));
}
/// tokio运行中时用block_on执行异步任务.用spawn在运行时中异步执行任务,spawn_blocking在线程池中执行阻塞任务.awaitJoinHandler等待异步任务结束.


///futures库是Rust库异步编程的基础抽象库,为编写异步代码提供核心的trait和类型.主要有如下功能:
/// - Future trait:表示一个异步计算的抽象,可通过.await获取其结果
/// - Stream trait: 表示一个异步的数据流,可通过.await迭代获取其元素
/// - Sink trait:代表一个可异步接收数据的目标
/// - Executor : 执行futures的运行时环境
/// - Utilities: 一些组合,创建futures函数


pub fn futures_async(){
    let pool = ThreadPool::new(Default::default()).expect("Failed to build pool");
    let (tx,rx) = mpsc::unbounded_channel()::<i32>();
}