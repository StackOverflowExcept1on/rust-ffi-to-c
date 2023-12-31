#[derive(Debug)]
struct MyString(String);

impl Drop for MyString {
    fn drop(&mut self) {
        println!("drop() called");
    }
}

extern "C-unwind" {
    fn add(a: i32, b: i32, callback: unsafe extern "C-unwind" fn(i32)) -> i32;
}

unsafe extern "C-unwind" fn f(ret: i32) {
    let local = MyString(String::from("41414141"));
    if ret == 4 {
        panic!();
    } else {
        println!("everything is fine");
    }
    println!("{:?}", &local.0);
}

fn main() {
    let result = std::panic::catch_unwind(|| {
        unsafe {
            add(2, 2, f);
        }
    });
    dbg!(result.is_err());
}
