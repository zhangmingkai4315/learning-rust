use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub fn fib_v1(n: usize) ->isize{
    match n {
        1|2 => 1,
        _ => fib_v1(n-1) + fib_v1(n-2)
    }
}

pub fn fib_v2(n: usize, hash_memo: &mut HashMap<usize, isize>)->isize{
    match n {
        1|2 => 1,
        _ => {
            match hash_memo.get(&n) {
                Some(v) => {
                    println!("found cached result for  {:?}",v);
                    *v
                },
                _ => {
                    let v = fib_v2(n-1, hash_memo) + fib_v2(n-2, hash_memo);
                    hash_memo.insert(n, v);
                    v
                }
            }
        }
    }
}


#[cfg(test)]
mod test{
    use crate::dynamic_program::fib::{fib_v1, fib_v2};
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn fib_v1_test(){
        assert_eq!(55, fib_v1(10));
    }

    #[test]
    fn fib_v2_test(){
        let mut memo: HashMap<usize, isize> = HashMap::new();
        assert_eq!(55, fib_v2(10, &mut memo));

        let mut memo: HashMap<usize, isize> = HashMap::new();
        assert_eq!(12586269025, fib_v2(50, &mut memo));
    }
}