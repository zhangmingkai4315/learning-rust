use std::collections::HashMap;

/// grid_traveller定义从一个n*m的格子中
/// 从左上到右下可能存在的路径, 要求只能向右和向下移动。
/// 比如下面的一个实例：2*3的格子，我们可以看到其路径总计有三条
///
/// +-----+------+-----+
/// |     |      |     |
/// |     |      |     |
/// +------------------+
/// |     |      |     |
/// |     |      |     |
/// +-----+------+-----+
///
/// [0,0] -> [0,1] ->[0-2] ->[1,2]
/// [0,0] -> [1,0] ->[1-1] ->[1,2]
/// [0,0] -> [0,1] ->[1-1] ->[1,2]
///
pub fn grid_traveller(n: usize, m: usize)-> usize{
    match(n, m){
        (n, m )if n == 0 || m == 0 => 0,
        (1,1) => 1,
        _ => grid_traveller(n-1, m) + grid_traveller(n, m-1),
    }
}


pub fn grid_traveller_memo(n: usize, m: usize, memo: &mut HashMap<String, usize>)-> usize{
    match(n, m){
        (n, m )if n == 0 || m == 0 => 0,
        (1,1) => 1,
        _ => {
            let key = format!("{},{}", n, m);
            match memo.get(key.as_str()){
               Some(v) => {
                   println!("found cached result for  {:?}",(n, m));
                   *v
               },
                None => {
                    println!("not found result for  {:?}",(n, m));
                    let result = grid_traveller_memo(n-1, m, memo) + grid_traveller_memo(n, m-1,memo);
                    memo.insert(key, result);
                    result
                }
            }

        },
    }
}

#[cfg(test)]
mod test {
    use crate::dynamic_program::grid_traveller::{grid_traveller, grid_traveller_memo};
    use std::collections::HashMap;

    #[test]
    fn grid_traveller_test() {
        assert_eq!(3, grid_traveller(2,3));
        assert_eq!(48620, grid_traveller(10,10));
    }


    #[test]
    fn grid_traveller_v2_test() {
        let mut memo = HashMap::new();
        assert_eq!(3, grid_traveller_memo(2,3, &mut memo));
        assert_eq!(20, grid_traveller_memo(4,4,&mut memo));
    }
}