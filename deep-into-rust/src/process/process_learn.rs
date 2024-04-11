use std::process::{Command, Stdio};

///Rust标准库std::process模块对进程进行操作.提供了创建,控制与外部进行交互的功能.
/// 创建进程:std::process::Command来创建新的进行,wait方法等待进行执行完成,将阻塞当前进程,直到进程完成.stdin,stdout,stderr方法配置进程的标准输入,标准输出和标准错误流
pub fn create_process_example(){
    use std::process::Command;
    let output = Command::new("ls").arg("-l").output().expect("Failed to execute command");
    println!("Output:{:?}", output);

    let mut child = Command::new("ls").spawn().expect("Failed to start command ls");
    let status = child.wait().expect("Failed to wait for command");
    println!("Command exited with:{:?}", status);

    let output1 = Command::new("echo").arg("Hello,Rust!").stdout(Stdio::piped()).output().expect("Failed to execute command");
    println!("Output1: {:?}", String::from_utf8_lossy(&output1.stdout));

}