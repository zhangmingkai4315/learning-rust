// error return local variable
// fn as_str(data: u32)->&str{
//     let s = format!("{}", data);
//     &s
// }


// fn as_str<'a>(data: &'a u32)->&'a str{
//     'b {
//         let s = format!("{}", data);
//         return &'a s;
//     }
// }

fn to_string(data: &u32) -> String{
    format!("{}",data)
}


fn compute(input: &u32, output: &mut u32){
    if *input > 10{
        *output += 1;
    }else if *input > 0{
        *output += 2;
    }
}


fn compute_optimize(input: &u32, output: &mut u32){
    let cached_input = *input;
    if cached_input > 10{
        *output += 1;
    }else if cached_input > 0{
        *output += 2;
    }
}





#[test]
fn test_fn(){

    let mut data = vec![1,2,3];
    let x = &data[0];
    println!("{}", x);
    data.push(4);

    // 传递新的数据，可能导致原数据结构被转移到新的位置。
    // 同时存在可变和不可变的借用，被禁止编译

    // assert_eq!(as_str(10), "10")
    let x:u32 = 10;
    let ref mut y:u32 = 0;
    compute(&x, y);
    assert_eq!(*y, 2)
}

