#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! capture {
    ($e: expr) => {
        match_tokens!($e)
    };
}

macro_rules! match_tokens {
    ($a: tt + $b: tt) => {
        "got an additional"
    };
    (($i:ident)) => {
        "got a identifier"
    };
    ($($other:tt)*) => {
        "got other "
    };
}

macro_rules! capture_then_what_is {
    (#[$m:meta]) => { what_is!(#[$m])};
}

macro_rules! what_is {
    (#[no_mangle]) => {"no_mangle"};
    (#[inline]) => {"inline"};
    ($($tty:tt)*) => {concat!("something else (", stringify!($($tty)*), ")")};
}

#[test]
fn test_capture() {
    // 当有多个宏一起使用的时候可能导致数据解析问题, 无法再次检查和匹配
    println!(
        "{}\n{}\n{}\n",
        capture!(3 + 6),
        capture!(5),
        capture!((caravan))
    );
    // got other
    // got other
    // got other
    println!(
        "{}\n{}\n{}\n",
        match_tokens!(3 + 6),
        match_tokens!(5),
        match_tokens!((caravan))
    );
    // got an additional
    // got other
    // got a identifier

    println!(
        "{}\n{}\n{}\n",
        what_is!(#[no_mangle]),
        what_is!(#[inline]),
        capture_then_what_is!(#[no_mangle])
    );
    // no_mangle
    // inline
    // something else (# [no_mangle])
}

// 使用crate来访问其他的对象
pub mod inner {
    #[macro_export]
    macro_rules! call_foo {
        () => {
            $crate::example02::inner::foo()
        };
    }
    pub fn foo() {
        println!("{}", "hello")
    }

    #[test]
    fn test_call() {
        crate::call_foo!()
    }
}
