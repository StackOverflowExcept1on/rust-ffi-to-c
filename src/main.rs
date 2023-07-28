#![feature(c_unwind)]

#[derive(Debug)]
struct MyString(String);

impl Drop for MyString {
    fn drop(&mut self) {
        println!("drop() called");
    }
}

extern "C" {
    fn add(a: i32, b: i32, callback: unsafe extern "C" fn(i32)) -> i32;
}

unsafe extern "C" fn f(ret: i32) {
    let local = MyString(String::from("41414141"));
    if ret == 4 {
        panic!();
    } else {
        println!("everything is fine");
    }
    println!("{:?}", &local.0);
}

fn main() {
    unsafe {
        add(2, 2, f);
    }
}
