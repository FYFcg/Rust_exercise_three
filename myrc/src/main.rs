use std::ops::Deref;

struct MyRc<T> {
    value: *mut T,
    count: *mut usize,
}

impl<T: Clone> MyRc<T> {
    fn new(value: T) -> Self {
        MyRc {
            value: Box::into_raw(Box::new(value)),
            count: Box::into_raw(Box::new(1)),
        }
    }

    fn clone(&self) -> Self {
        unsafe {
            *self.count += 1;
        }
        MyRc {
            value: self.value,
            count: self.count,
        }
    }

    fn get_count(&self) -> usize {
        unsafe { *self.count }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.count -= 1;
            if *self.count == 0 {
                println!("Dropping!");
                drop(Box::from_raw(self.value));
                drop(Box::from_raw(self.count));
            }
        }
    }
}

fn main() {
    let rc1 = MyRc::new(String::from("hello"));
    println!("rc1 = {}", *rc1);
    println!("count of rc1 after creating rc1: {}", rc1.get_count());
    {
        let rc2 = rc1.clone();
        println!("rc2 = {}", *rc2);
        println!("count of rc1 after cloning rc1: {}", rc1.get_count());
        println!("count of rc2 after cloning rc1: {}", rc2.get_count());
    }
    println!("count of rc1 after dropping rc2: {}", rc1.get_count());
}
