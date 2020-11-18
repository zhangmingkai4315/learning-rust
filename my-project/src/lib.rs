// use std::env;
// use std::collections::HashMap;
//
// use std::future::{self, Future};


mod front_of_house;

pub use crate::front_of_house::hosting;

fn eat_at_resaurant(){
    hosting::add_to_waitlist();
}


pub fn divide(a: i32, b: i32)->i32{
    println!("receive {} / {}", a, b);
    a/b
}

#[cfg(test)]
mod tests{
    use crate::divide;
    use std::fmt::Error;

    #[test]
    fn it_works(){
        assert_eq!(divide(4,2), 2)
    }

    #[test]
    #[should_panic]
    fn should_panic_test(){
        let _ = divide(20, 0);
    }

    #[test]
    #[ignore]
    fn return_result_test() -> Result<(), String>{
        if divide(4,2) != 2{
            Err("divide not work".to_string())
        }else{
            Ok(())
        }
    }

}

