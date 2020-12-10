fn fib_v1(n: usize)->isize{
    match n {
        1|2 => 1,
        _ => fib_v1(n-1) + fib_v1(n-1)
    }
}


#[cfg(test)]
mod test{
    use crate::dynamic_program::fib::fib_v1;

    #[test]
    fn fib_v1_test(){
        println!(fib_v1(10));
    }
}