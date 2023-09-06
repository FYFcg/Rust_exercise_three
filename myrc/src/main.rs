use std::cell::RefCell;

struct MyRc<T> {
    value: T,
    count: RefCell<usize>,
}

impl<T:Clone> MyRc<T> {
    fn new(value: T) -> Self {
        MyRc {
            value: value,
            count: RefCell::new(1),
        }
    }

    fn clone(&self) -> Self {
        *self.count.borrow_mut() += 1;
        MyRc {
            value: self.value.clone(),
            count: self.count.clone(),
        }
    }

    fn get_count(&self) -> usize {
        *self.count.borrow()
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let count = &mut *self.count.borrow_mut();
        *count -= 1;
        if *count == 0 {
            println!("Dropping MyRc");
        }
    }
}

fn main() {
    let rc1 = MyRc::new(String::from("hello"));
    println!("count after creating rc1: {}", rc1.get_count());

    let rc2 = rc1.clone();
    println!("count after cloning rc1: {}", rc1.get_count());

    drop(rc1);
    println!("count after dropping rc1: {}", rc2.get_count());

    drop(rc2);
}
