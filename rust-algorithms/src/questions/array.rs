use std::mem::swap;

// 顺时针旋转数组K个位置 可以先获得%的归一化数值
// 整个数组进行旋转
// 旋转前半段+旋转后半段 以偏移为中点
fn rotate(nums: &mut Vec<i32>, k: i32) {
    if k <= 0 {
        return
    }
    let k = k as usize;
    let size = nums.len();
    let shift = k % size;
    if size < 2 || shift == 0{
        return
    }
    reverse(nums, 0, size-1);
    reverse(nums, 0, shift-1);
    reverse(nums, shift , size-1);
}

fn reverse(nums: &mut Vec<i32>, start: usize, end: usize){
    let mut i = start;
    let mut j = end;
    while i < j {
        nums.swap(i, j);
        i+=1;
        j-=1;
    }
}

#[cfg(test)]
mod my_test{
    use super::{rotate,reverse};
    #[test]
    fn test_reverse() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let size = nums.len();
        reverse(&mut nums, 0, 4);
        assert_eq!(nums, vec![5, 4, 3, 2, 1]);

        let mut nums = vec![1];
        reverse(&mut nums, 0, 0);
        assert_eq!(nums, vec![1]);

        let mut nums = vec![1,2];
        reverse(&mut nums, 0, 1);
        assert_eq!(nums, vec![2, 1]);

        let mut nums = vec![5,4,3,2,1];
        reverse(&mut nums, 1, 4);
        assert_eq!(nums, vec![5, 1,2,3,4]);
    }
    #[test]
    fn test(){
        // let mut nums = vec![1,2,3,4,5];
        // rotate(&mut nums,1);
        // assert_eq!(nums, vec![5,1,2,3,4]);
        //
        // let mut nums = vec![1,2,3,4,5];
        // rotate(&mut nums,3);
        // assert_eq!(nums, vec![3,4,5,1,2]);

        let mut nums = vec![1,2 ];
        rotate(&mut nums,2);
        assert_eq!(nums, vec![1, 2 ]);
    }
}