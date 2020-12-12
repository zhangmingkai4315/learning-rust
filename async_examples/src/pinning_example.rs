use std::pin::Pin;

#[derive(Debug)]
struct Test{
    a: String,
    b: *const String,
}

impl Test{
    fn new(text: &str)->Test{
        Test{
            a: text.to_owned(),
            b: std::ptr::null(),
        }
    }
    fn init(&mut self){
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }
    fn get_a(&self) -> &str{
        &self.a
    }
    fn get_b(&self) -> &String{
        unsafe {
            &*self.b
        }
    }
}

#[test]
fn test(){
    let mut test1 = Test::new("hello");
    let mut test2 = Test::new("world");
    test1.init();
    test2.init();

    println!("a={} b={}", test1.get_a(), test1.get_b());
    std::mem::swap(&mut test1, &mut test2);
    println!("a={} b={}", test2.get_a(), test2.get_b());

    // a=hello b=hello
    // a=hello b=world

}


use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test2 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test2{
    fn new(text: &str) -> Self{
        Test2{
            a: String::from(text),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>){
        let self_ref: *const String = &self.a;
        let this = unsafe{
            self.get_unchecked_mut()
        };
        this.b = self_ref;
    }

    fn get_a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }
    fn get_b(self: Pin<&Self>) -> &String{
        unsafe {
            &*(self.b)
        }
    }
}

#[test]
fn test_2(){
    let mut test1 = Test2::new("hello");
    let mut test1 = unsafe{
        Pin::new_unchecked(&mut test1)
    };
    let mut test2 = Test2::new("world");
    let mut test2 = unsafe{
        Pin::new_unchecked(&mut test2)
    };

    Test2::init(test1.as_mut());
    Test2::init(test2.as_mut());

    println!("a: {}, b: {}", Test2::get_a(test1.as_ref()), Test2::get_b(test1.as_ref()));
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", Test2::get_a(test2.as_ref()), Test2::get_b(test2.as_ref()));

    // a=hello b=hello
    // a=hello b=world

}


#[derive(Debug)]
struct Test3 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test3 {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test3 {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_3(){
    let mut test1 = Test3::new("test1");
    let mut test2 = Test3::new("test2");

    println!("a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b());
}