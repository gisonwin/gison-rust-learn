use std::process::{Command, Stdio};

///Rust标准库std::process模块对进程进行操作.提供了创建,控制与外部进行交互的功能.
/// 创建进程:std::process::Command来创建新的进行,wait方法等待进行执行完成,将阻塞当前进程,直到进程完成.stdin,stdout,stderr方法配置进程的标准输入,标准输出和标准错误流
pub fn create_process_example() {
    use std::process::{Command, Stdio};
    let output = Command::new("ls").arg("-l").output().expect("Failed to execute command");
    println!("Output:{:?}", output);

    let mut child = Command::new("ls").spawn().expect("Failed to start command ls");
    let status = child.wait().expect("Failed to wait for command");
    println!("Command exited with:{:?}", status);
    //stdout被配置为管道,读取进程里的输出.
    let output1 = Command::new("echo").arg("Hello,Rust!").stdout(Stdio::piped()).output().expect("Failed to execute command");
    println!("Output1: {:?}", String::from_utf8_lossy(&output1.stdout));

    //env方法可以设置进程的环境变量,此处 我们设置MY_VAR 这个变量
    let output2 = Command::new("printenv").env("MY_VAR", "HelloRust").output().expect("Failed to execute command");
    println!("Output2: {:?}", String::from_utf8_lossy(&output2.stdout));
    //设置工作目录 current_dir,进程将在指定文件夹中进行,而不是当前Rust程序的工作目录.对于确保进程在正确的环境中执行非常有用,特别是依赖于相对路径的操作.
    let output3 = Command::new("ls").arg("-l").current_dir("/path/to/your/directory").output().expect("Failed to execute command");
    println!("output3:{:?}", String::from_utf8_lossy(&output3.stdout));
    //uid,gid分别是设置进程的用户标识和组标识.注意这里需要管理员身份或有足够的权限来更改进程的用户标识 和组标识
    // let output4 = Command::new("whoami").uid(1000).gid(1000).output().expect("Failed to execute command");
    // println!("output4:{:?}", String::from_utf8_lossy(&output4.stdout));
}


//传递给子进程打开的文件,实现子进程恢复套接字的代码涉及到子进程解析命令行参数并使用nix库的dup2函数将文件描述符复制到正确的位置.

// pub fn fork_child_process_example() {
//     use std::net::{TcpListener, TcpStream};
//     use std::os::windows::io::AsRawSocket;
//     use nix::unistd::{dup2, close, ForkResult};
//     let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
//     let (stream, _) = listener.accept().expect("Failed to accept connection");
//     let socket_fd = stream.as_raw_socket();
//     let backup_fd = dup2(socket_fd, 10).expect("Failed to duplicate file");
//     //将套接字传递给子进行
//     match unsafe { nix::unistd::fork() } {
//         Ok(ForkResult::Parent { child }) => {
//             close(backup_fd).expect("Failed to close backup file desc");
//             println!("Parent process.Child PID:{child}");
//             println!("Parent process.Child PID:{:?}", child);
//         }
//         Ok(ForkResult::Child) => {
//             dup2(backup_fd, socket_fd).expect("Failed to restore file desc");
//             close(backup_fd).expect("Failed to close backup fd");
//             let mut buffer = [0; 1024];
//             let _ = TcpStream::from_raw_fd(socket_fd).read(&mut buffer);
//             println!("Child process.Received:{:?}", String::from_utf8_lossy(&buffer));
//         }
//         Err(_) => {
//             eprintln!("Fork failed");
//         }
//     }
// }

///std::process::Child类型可以控制子进程,std::process::Command.spawn()返回Child,并提供一些方法来与子进程进行交互,等待其结束以及发送信号等.

pub fn control_child_process_example() {
    //等待子进程结束
    let mut child = Command::new("echo").arg("Hello,Rust!").spawn().expect("Failed to start command");
    let status = child.wait().expect("Failed to wait for command");
    println!("Command exited with:{:?}", status);
    //向子进程发送信号
    let mut child2 = Command::new("sleep").arg("10").stdout(Stdio::null()).spawn().expect("Failed to start command");
    child.kill().expect("Failed to send signal");
    //通过标准输入输出与子进程交互
    use std::io::Write;
    let mut child3 = Command::new("cat").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Failed to start command");
    if let Some(mut stdin) = child3.stdin.take() {
        stdin.write_all(b"Hello,Rust!\n").expect("Failed to write to stdin");
    }
    let output = child3.wait_with_output().expect("Failed to wait for command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
    //echo "hello,rust!" |grep Rust这个命令会创建两个子进程,一个作为生产者,一个作为消费者.生产者进程将数据写入管道,消费者进程从管道中读取数据.
}

pub fn pipe_example() {
    //producer process
    let producer = Command::new("echo").arg("Hello Rust").stdout(Stdio::piped()).spawn().expect("Failed to start producer command");
    //consumer process
    let consumer = Command::new("grep").arg("Rust").stdin(producer.stdout.unwrap()).output().expect("Failed to start consumer command");

    let output = String::from_utf8_lossy(&consumer.stdout);
    println!("Output:{output}");

    let command = "echo \"Hello,Rust!\" |grep Rust";
    let output1 = Command::new("sh").arg("-c").arg(command).output().expect("Failed to execute command");
    let cow = String::from_utf8_lossy(&output1.stdout);
    println!("Output1:{cow}");
}

///Stdio::piped()是stdio的枚举成员,它表示在创建子进程时候创建一个管道,并将其用于标准输入,标准输出或标准错误.创建的管道将其连接到子进程的标准输出,我们再从子进程的标准输出读取数据.
/// Stdio::null()表示一个特殊的标准输入,输出或标准错误,即空设备(null device),Unix-like系统中,空设备通常被表示为/dev/null,任何写入它的数据都会被丢弃,任何尝试从中读取的操作都
///会立即返回EOF(End Of File,文件结束符).Rust中Stdio::null()用于将标准输入,标准输出或标准错误连接到空设备,即忽略相关的输入或输出.当你想要禁用子进程的输出或输入时很有用.

pub fn stdio_null_example() {
    //子进程的标准输出被连接到空设备,输出被丢弃了.父进程等待子进程结束后,并输出子进程的退出状态.
    let mut child = Command::new("echo").arg("Hello,Rust!").stdout(Stdio::null()).spawn().expect("Failed to start command");
    let status = child.wait().expect("failed to wait for command");
    println!("{status}");
}