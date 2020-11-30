use std::collections::{HashMap, HashSet};

/// 假设所需要处理的数组为一个不包含重复元素的数组
/// 我们通过使用排序的方式，将输出进行预处理
/// 处理后的数值分别从左右两个方向进行结合 直到遇到合适的数据
/// 时间复杂度为O(nlogn)
fn sum_with_target(list: &mut [u32], target: u32) ->bool{
    let size = list.len();
    match size {
        0|1 => return false,
        2 => return list[1] + list[0] == target,
        _ => {},
    }
    list.sort();

    let mut i = 0;
    let mut j = size-1;
    while i<j {
        if list[i] + list[j] == target{
            return true;
        }
        if list[i] + list[j] > target{
            j-=1;
        }else{
            i+=1;
        }
    }
    return false;
}





/// 使用hash可以保证O(n)的时间复杂度，但是空间占用相对原地的比较
/// 需要额外的存储。此方法也只限于非重复元素的列表
fn sum_with_hash(list: &mut [u32], target: u32)->bool {
    let size = list.len();
    match size {
        0|1 => return false,
        2 => return list[1] + list[0] == target,
        _ => {},
    }
    let mut hash_set = HashSet::new();
    for i in list{
        match hash_set.get(&(target - *i)) {
            Some(v) => return true,
            _ => hash_set.insert(i),
        };
    }
    return false;
}


/// 同上面 但是需要返回下标
fn sum_with_hash_v2(list: &mut [u32], target: u32)->Option<[usize;2]> {
    let size = list.len();
    match size {
        0|1 => return None,
        _ => {},
    };

    let mut hash_map = HashMap::new();
    for (i,v) in list.iter().enumerate(){
        hash_map.insert(v, i);
    }
    for (i,v) in list.iter().enumerate(){
        match hash_map.get(&(target - *v)) {
            Some(j) => return Some([i, *j]),
            _ => {},
        };
    }
    return None;
}



#[cfg(test)]
mod test{
    use crate::questions::two_sum::{sum_with_target, sum_with_hash, sum_with_hash_v2};

    #[test]
    fn test_two_sum(){
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_target(list.as_mut_slice(), 9), true);
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_target(list.as_mut_slice(), 15), true);
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_target(list.as_mut_slice(), 5), true);
        let mut list = vec![1,4];
        assert_eq!(sum_with_target(list.as_mut_slice(), 5), true);

        let mut list = vec![1,1,2,4];
        assert_eq!(sum_with_target(list.as_mut_slice(), 2), true);

        let mut list = vec![];
        assert_eq!(sum_with_target(list.as_mut_slice(), 9), false);
        let mut list = vec![1];
        assert_eq!(sum_with_target(list.as_mut_slice(), 9), false);
        let mut list = vec![1,4];
        assert_eq!(sum_with_target(list.as_mut_slice(), 9), false);
    }

    #[test]
    fn test_two_sum_with_sum(){
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 9), true);
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 15), true);
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 5), true);
        let mut list = vec![1,4];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 5), true);

        let mut list = vec![1,1,2,4];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 2), true);

        let mut list = vec![];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 9), false);
        let mut list = vec![1];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 9), false);
        let mut list = vec![1,4];
        assert_eq!(sum_with_hash(list.as_mut_slice(), 9), false);
    }

    #[test]
    fn test_two_sum_with_hash_v2(){
        let mut list = vec![1,4,2,6,3,9];
        assert_eq!(sum_with_hash_v2(list.as_mut_slice(), 9), Some([3,4]));
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_hash_v2(list.as_mut_slice(), 15), Some([3,5]));
        let mut list = vec![1,4,2,6,3,9,5,7,10];
        assert_eq!(sum_with_hash_v2(list.as_mut_slice(), 5), Some([0,1]));

        let mut list = vec![];
        assert_eq!(sum_with_hash_v2(list.as_mut_slice(), 9), None);
        let mut list = vec![1];
        assert_eq!(sum_with_hash_v2(list.as_mut_slice(), 9), None);
        let mut list = vec![1,4];
        assert_eq!(sum_with_hash_v2(list.as_mut_slice(), 9), None);
    }
}