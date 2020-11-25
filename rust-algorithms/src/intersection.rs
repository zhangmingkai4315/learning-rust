
pub mod intersection{
    pub fn intersection_array_v1(array1: &Vec<i32>, array2: &Vec<i32>) ->Vec<i32>{
        let mut result :Vec<i32> = vec![];
        for v1 in array1.iter(){
            'inner: for v2 in array2.iter(){
                if v1 < v2 {
                    break 'inner
                }else if v1 > v2 {
                    continue 'inner
                }else{
                    result.push(v1.clone())
                }
            }
        }
        result
    }

    pub fn intersection_array_binary_search(array1: &Vec<i32>, array2: &Vec<i32>) -> Vec<i32>{
        let (small_arr, big_arr) = {
            if array1.len()>array2.len(){
                (array2, array1)
            }else{
                (array1, array2)
            }
        };
        let mut result :Vec<i32> = vec![];
        let mut offset = 0;
        for i in small_arr.iter(){
            let (found, index) = binary_search(*i, &big_arr[offset..]);
            if index == big_arr.len(){
                break;
            }
            if found == true{
                result.push(i.clone());
            }
            offset = index
        }
        result
    }
    /// binary_search 返回一个tuple，其中bool表示是否找到元素，usize 表示其当前的位置
    /// 如果未找到则usize表示当前的低位偏移位
    pub fn binary_search(target: i32, arr: &[i32]) -> (bool, usize){
        let len = arr.len();
        let mut low = 0;
        let mut high = len - 1;
        while low<=high {
            let mid = (high-low)/2 + low;
            let val = arr[mid];

            if val < target{
                low = mid + 1;
            }else if val > target{
                if mid < 1{
                    break
                }
                high = mid - 1;
            }else{
                return (true, mid);
            }
        }
        (false, low)
    }
}


#[cfg(test)]
mod tests {
    use super::intersection::*;

    #[test]
    fn test_intersection(){
        let arr1 = vec![1,2,3,4,5,6];
        let arr2 = vec![4,5,7,7,9];

        let result = intersection_array_v1(&arr1, &arr2);
        assert_eq!(result, vec![4,5]);

        let arr1 = vec![0,6];
        let arr2 = vec![4,5,7,7,9];

        let result = intersection_array_v1(&arr1, &arr2);
        assert_eq!(result, vec![]);

        let arr1 = vec![];
        let arr2 = vec![4,5,7,7,9];

        let result = intersection_array_v1(&arr1, &arr2);
        assert_eq!(result, vec![]);

        let arr1 = vec![4,5,5,7,8];
        let arr2 = vec![];

        let result = intersection_array_v1(&arr1, &arr2);
        assert_eq!(result, vec![]);

        let arr1 = vec![];
        let arr2 = vec![];

        let result = intersection_array_v1(&arr1, &arr2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_binary_search(){
        let arr1 = vec![1,2,3,4,5,6,9,10,20];
        assert_eq!(binary_search(10, &arr1[..]), (true, 7));
        assert_eq!(binary_search(5, &arr1[..]), (true, 4));
        assert_eq!(binary_search(-1, &arr1[..]), (false, 0));
        assert_eq!(binary_search(200, &arr1[..]), (false, 9));
    }

    #[test]
    fn test_intersection_bst(){
        let arr1 = vec![1,2,3,4,5,6];
        let arr2 = vec![4,5,7,7,9];

        let result = intersection_array_binary_search(&arr1, &arr2);
        assert_eq!(result, vec![4,5]);

        let arr1 = vec![0,6];
        let arr2 = vec![4,5,7,7,9];

        let result = intersection_array_binary_search(&arr1, &arr2);
        assert_eq!(result, vec![]);

        let arr1 = vec![];
        let arr2 = vec![4,5,7,7,9];

        let result = intersection_array_binary_search(&arr1, &arr2);
        assert_eq!(result, vec![]);

        let arr1 = vec![4,5,5,7,8];
        let arr2 = vec![];

        let result = intersection_array_binary_search(&arr1, &arr2);
        assert_eq!(result, vec![]);

        let arr1 = vec![];
        let arr2 = vec![];

        let result = intersection_array_binary_search(&arr1, &arr2);
        assert_eq!(result, vec![]);
    }
}