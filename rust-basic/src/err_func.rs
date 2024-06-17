#[derive(Debug)]
pub struct MyError {
    pub detail: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display Error:{}", self.detail)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

fn errs(my_err: String) -> Result<(), MyError> {
    Err(MyError {
        detail: my_err,
    })
}

pub fn func_errs(my_error: MyError) {
    match errs(my_error.detail) {
        Ok(_) => { println!("Ok") }
        Err(err) => { println!("111Error:{}", err) }
    }
}

pub fn func_errs_char() -> Result<(), MyError> {
    errs("aaaaa".to_owned())?;
    Ok(())
}

/// borrow checker 原则
/// 1 不可变引用规则:任何给定时间,要么有一个可变引用,要么有多个不可变引用,但不能同时存在可变引用和不可变引用,确保同一时间只有一个地方对数据进行修改,或多个地方同时读取数据.
/// 2 可变引用规则:任何给定时间,只能有一个可变引用来访问数据.防止了并发修改相同数据问题,从而防止数据竞争
/// 3 引用的生命周期必须在被引用的数据有效时间范围内,防止了悬垂引用,即引用数据已被销毁,但引用仍然存在
/// 4 可变引用与不可变引用互斥:可同时存在多个不可变引用,因为不可变引用不会修改数据,不会影响到其他引用,但不可变引用与可变引用之间是互斥的.
/// 大多数情况下生命周期是隐式且被推断的.其主要目的就是防止悬垂引用.