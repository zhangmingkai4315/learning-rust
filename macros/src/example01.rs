macro_rules! four {
    () => {
        1 + 3
    };
}

#[test]
fn test_macro_four() {
    println!("{}", four!());
    println!("{}", four![]);
    println!("{}", four! {});
}

macro_rules! vec_str {
    ( $($elem: expr),*) => {
        {
            let mut v = Vec::new();

            $(
                v.push(format!("{}",$elem));
            )*

            v
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_macro_vec_str() {
        let s = vec_str![10, "dns", "abc", 10.1];
        assert_eq!(s, vec!["10", "dns", "abc", "10.1"])
    }
}

// --------------------

// item 匹配任何rust的item信息
macro_rules! items {
    ($($item: item)*) => {};
}

#[test]
fn test_marco_item() {
    items!(
        struct Foo;
        enum Bar{}
        impl Foo{}
    );
}

// item 匹配任何rust的block 以大括号包裹
macro_rules! blocks {
    ($($item: block)*) => {};
}
#[test]
fn test_marco_blocks() {
    blocks!(
        {}
        {let hello;}
        { 2 }
    );
}

// item 匹配任何rust的statement
macro_rules! stms_example {
    ($($item: stmt)*) => ($($item)*);
}

#[test]
fn test_marco_stsm() {
    stms_example!(
        struct Foo;
        fn foo(){}
        let zig = 3;
    );
}

// item 匹配任何rust的statement
macro_rules! pattern_example {
    ($($item: stmt)*) => ($($item)*);
}

#[test]
fn test_marco_pattern() {
    pattern_example!(
        struct Foo;
        fn foo(){}
        let zig = 3;
    );
}

// path ...
macro_rules! paths {
    ($($path:path)*) => {};
}
#[test]
fn test_marco_path() {
    paths!(
        ::A::B::C::D
        G::A
        SimplePath
    );
}

// identifier
macro_rules! ident {
    ($($path:ident)*) => {};
}
#[test]
fn test_marco_ident() {
    ident!(
        foo
        bar
        // _ // ^ no rules expected this token in macro call
        async
    );
}

// types
macro_rules! types {
    ($($path:ty)*) => {};
}
#[test]
fn test_marco_types() {
    types!(
        foo::bar
        bool
        [u8]
    );
}

// types
macro_rules! exprs {
    ($($path:expr)*) => {};
}
#[test]
fn test_marco_exprs() {
    exprs!(
        "hello"
        funcall()
        future.await
        break 'a
    );
}

// lifetimes
macro_rules! lifetimes {
    ($($path:lifetime)*) => {};
}
#[test]
fn test_marco_lifetimes() {
    lifetimes!(
        'static
        '_
        'hello
    );
}
