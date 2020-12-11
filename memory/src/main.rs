enum Name{
    Anonymous,
    NickName(String),
    FullName{ first: String, last: String},
}

// 12 byte
enum BorderStyleValue{
    Solid,
    Dashed(Option<u32>)
}
// 16 byte
type BorderStyle = Option<BorderStyleValue>;

// 8byte
enum BorderStyleValueV2{
    None,
    Solid,
    Dashed(u32)
}
// 12 byte
type BorderStyleV2 = Option<BorderStyleValue>;

// 8 byte
enum BorderStyleV3 {
    None, Solid, DashedNone, Dashed(u32)
}

//  rustc --emit asm src/main.rs

fn main() {
    // 56
    // 0 / 24 / 48 + 2(padding to 8) = 56
    println!("{}",std::mem::size_of::<Name>());
}
