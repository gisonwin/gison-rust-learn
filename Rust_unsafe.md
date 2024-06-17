# Rust中Unsafe代码

Rust语言哲学中安全优先是其核心原则之一，但仍然提供了unsafe 关键字，unsafe代码允许开发者暂时脱离Rust的安全限制，直接操作内存和执行低级操作。虽然unsafe代码有些情况下是必要的，但使用它时必须格外小心，以避免引入难以调试的内存错误。

### 什么是unsafe代码

Rust中unsafe用于标记哪些可能破坏Rust的内存安全保证的代码块，使用unsafe关键字编写的代码块或函数被移为unsafe代码。

unsafe代码允许使用者执行如祼指针操作、类型转换和直接内存访问等低级操作。这些操作可能导致未定义行为或内存安全漏洞，Rust编译器不会对它们进行常规安全性检查。

### 应用场景：

- 性能优化：某些性能关键的应用中，使用者使用unsafe代码绕过Rust安全性检查来以获得更高的性能。
- 底层系统编程：在操作系统开发、设备驱动或嵌入式系统编程中，可能需要直接操作硬件或使用特定的内存布局，这时就需要使用unsafe代码。
- 与C语言库交互 :当Rust调用C语言库时，可能需要执行一些不安全的操作来正确地管理内存和调用约定。

### Rust中unsafe代码使用

主要涉及到以下三个方面：使用裸指针、使用外部函数接口、实现不安全trait.

#### 使用裸指针：

   Rust中，裸指针是一种可绕过Rust常规所有权和借用检查机制的低级工具，它允许使用者直接操作内存地址，从而进行更为底层和灵活的操作，使用时必须格外小心，以避免引入未定义行为或内存安全问题。

 裸指针有两种主要类型：\*const T(指向常量数据的裸指针) 和  \*mut T (指向可变数据的裸指针)。前者用于读取数据，后者用于读取和修改数据。

裸指针通过取址操作符&和类型转换来创建。下面我们首先创建一个整数x和一个可变整数y，然后使用取址操作符&获取它们的地址，并通过类型转换将它们转换为裸指针raw_ptr,mut_raw_ptr.获取裸指针代码并不是unsafe代码，解引用裸指针才是unsafe代码。

 解引用裸指针是通过在裸指针前使用\*操作符完成，这允许我们读取或修改裸指针指向的值。注意：解引用裸指针时必须确保指针是有效的，否则会导致未定义行为。

下面我们使用unsafe块来解引用裸指针，在unsafe块内我们打印出raw_ptr指向的值，并将mut_raw_ptr指向的值修改为1024。

```rust
fn main(){
    let x=66;
    let raw_ptr:*const i32 = &x as *const i32;
    
    let mut y = 99;
    let mut_raw_ptr:*mut i32 = &mut y as *mut i32;
    unsafe{
        println!("{}",*raw_ptr);
        *mut_raw_ptr = 1024;
        println!("{}",*mut_raw_ptr);
    }
}
```



#### 使用外部函数接口:

​		Rust中调用C语言或其他语言编写的库函数时使用unsafe标记。Rust通过extern块和extern关键字提供了对外部函数的支持，这些函数通常需要标记为unsafe.这是因为Rust编译器无法验证这些外部函数行为是否符合Rust的内存安全规则。

​	 假设我们有下面的C语言库，其Add接口为计算两个整数的和。

```c
//Add.h
#ifdef __cplusplus
extern "C"{
#endif
    int add(int a,int b);
 #ifdef __cplusplus
}
#endif

//Add.c
#include "Add.h"
int Add(int a ,int b)
{
    return a+b;
}
```

Rust中提供了一个包含C语言类型的库，如libc，使得我们可以使用与C兼容的类型。然后我们使用extern ”C“块来声明C语言中的Add函数。extern “C”告诉Rust编译器这个函数是用C语言的链接约定来链接的。Rust main函数中使用unsafe块来调用这个外部函数，如果C函数违反了这些规则(如解引用空指针或写入只读内存)，Rust可能会产生崩溃或产生未定义行为。最后编译和运行这个Rust程序需要确保实现Add函数的C库是可用的，我们需要编译这个C库为动态链接库或静态库，并在编译Rust程序时链接这个库。我们还需要在cargo.toml中添加以下依赖: libc=“0.2”

```rust
use libc::{c_int};
extern "C"{
    fn Add(a:c_int,b:c_int) -> c_int;
}
fn main(){
    unsafe{
        let sum= add(66,77);
        println!("{}",sum);
    }
}
```

#### 实现不安全trait:

Rust中可直接声明一个不安全的trait,即整个trait都带有unsafe修饰符，也可不声明不安全的trait，但在trait的具体实现中使用unsafe来执行不安全的操作。这意味着我们可以安全地定义一个trait，但在其某个或某些实现中执行不安全的操作。

下例中 UnsafeTrait声明一个unsafe_method方法，CustomStruct实现了这个trait,并提供了unsafe_method的默认实现，该实现是unsafe的。main函数中我们使用unsafe块来调用这个方法。即使unsafe_method是在trait中定义的，调用它的责任仍然落在调用者身上。调用者必须确保在调用unsafe方法时遵循所有安全准则，如确保传递给方法的参数是有效的、并处理像任何可能由unsafe操作引起的错误或未定义行为。

```rust
traint UnsafeTrait{
    unsafe fn unsafe_method(&self) ->Result<(),String>;
}
struct CustomStruct;
impl UnsafeTrait for CustomStruct{
    unsafe fn unsafe_method(&self) -> Result<(),String>{
        //omit some operate
        Ok(())
    }
}
fn main(){
    let my_struct = CustomStructor{};
    unsafe{
        match my_struct.unsafe_method{
            Ok(())=> println!("success"),
            Erro(e) => println!("failed:{}",e),
        }
    }
}
```

所有尽量避免在trait中使用unsafe，除非一些必须执行的低级，不安全的操作，且调用者能够清楚地理解并处理这些不安全操作可能带来的风险。

### unsafe代码的安全抽象

unsafe代码的安全抽象是一种设计模式，它允许开发者在不安全代码和安全代码之间建立清晰的边界。这种抽象通过封装不安全操作在安全的接口之后来实现，使得库使用者能够在不了解或不关心内部实现细节情况下安全地使用库的功能。

该设计模式关键在于：将不安全代码限制在尽可能小的范围内，并通过安全接口暴露给使用者。这样库使用者可以依赖这些安全的接口而无须担心底层可能的不安全操作。

下例中unsafe_operation函数执行一些不安全操作，但它并没有直接公开给库的使用者，而是被封装在safe_operation函数中，safe_operation函数是一个安全的接口，它内部使用unsafe块来调用 unsafe_operation函数，但在调用前后可以添加额外的安全检查或清理工作。这样库的使用者只需要调用safe_operation即可，而无需关心其内部是否使用unsafe。

```rust
unsafe fn unsafe_operation(){
    
}
pub fn safe_operation(){
    //before
    unsafe{
        unsafe_operation();
    }
    //after 可添加额外的安全检查或清理工作
}
fn main(){
    safe_operation();
}
```

通过安全抽象这种方式，库设计者可确保库的使用者不会误用不安全的操作，同时仍然能够利用不安全代码提供的性能优势或底层功能。在构建大型 Rust项目或库时，将不安全代码限制在最小的必要范围内，并通过安全接口暴露功能是非常必要的，有助于减少错误和漏洞的风险，同时提高代码的可维护性和可读性。

### 注意事项

虽然unsafe并非不完全受控，但它确实把内存安全交给使用者，所以编写unsafe代码时需要注意以下几点：

1. 最小化unsafe代码的使用。尽量将unsafe代码使用限制在必要范围内，并尽量避免在库或模块的公共API中使用它。
2. 仔细审查unsafe代码块，对代码块进行严格审核和测试，以确保它不会引入内存安全漏洞。
3. 文档化unsafe代码，为unsafe代码提供清晰的文档说明，解释为什么使用它，以及使用它有哪些需要注意事项。
4. 使用Rust的安全抽象模式。尽可能利用Rust提供的所有权模型、生命周期和借用检查器等安全抽象来减少unsafe代码的使用。