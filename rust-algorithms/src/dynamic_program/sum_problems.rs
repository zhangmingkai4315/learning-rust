use std::collections::HashMap;
fn can_sum(target: isize, numbers: &Vec<isize>) -> bool{
    match target{
        x if x < 0 => false ,
        x if x == 0 => true,
        _ => {
            for number in numbers{
                if can_sum(target-number, numbers) == true {
                    return true ;
                }
            }
            return false ;
        }
    }
}

#[test]
fn test_can_sum(){
    assert_eq!(true, can_sum(7, &vec![1,4,5]));
    assert_eq!(true, can_sum(7, &vec![2,4,5]));
    assert_eq!(false, can_sum(7, &vec![4,5]));
    // assert_eq!(true, can_sum(300, &vec![7,14]));
}

fn can_sum_memo(target: isize, numbers: &Vec<isize>, memo: &mut HashMap<isize, bool>) -> bool{
    match target{
        x if x < 0 => false ,
        x if x == 0 => true,
        _ => {
            if let Some(v) = memo.get(&target) {
               return false
            }
            for number in numbers{
                if can_sum_memo(target-number, numbers, memo) == true {
                    return true ;
                }else{
                    memo.insert(target-number, false);
                }
            }
            return false ;
        }
    }
}

#[test]
fn test_can_sum_memo(){
    let mut memo = HashMap::new();
    assert_eq!(true, can_sum_memo(7, &vec![1,4,5], &mut memo));
    let mut memo = HashMap::new();
    assert_eq!(true, can_sum_memo(7, &vec![2,4,5],&mut memo));
    let mut memo = HashMap::new();
    assert_eq!(false, can_sum_memo(7, &vec![4,5],&mut memo));

    let mut memo = HashMap::new();
    assert_eq!(false, can_sum_memo(300, &vec![7,14],&mut memo));
}

/// How sum will get the slice of how to sum to targets

fn how_sum_memo(target: isize, numbers: &Vec<isize>, memo: &mut HashMap<isize, bool>) -> Option<Vec<isize>>{
    match target{
        x if x < 0 => None ,
        x if x == 0 => Some(vec![]),
        _ => {
            match memo.get(&(target)){
                None => {}
                Some(v) => {
                    // 存入的只有None对象
                    return None
                }
            }
            for number in numbers{
                let key = target - number;
                if key < 0{
                    continue
                }
                let sum_result = how_sum_memo(key, numbers, memo);
                match sum_result{
                    Some(mut v) => {
                        if v.len() >= 0{
                            let mut new_vec:Vec<isize> = [*number].to_vec();
                            v.push(*number);
                            return Some(v);
                        }
                    },
                    _ => {
                        // 只缓存处理后不存在序列的情况
                        memo.insert(key, false);
                    },
                };
            }
            return None ;
        }
    }
}



#[test]
fn test_how_sum_memo(){
    let mut memo = HashMap::new();
    let result = how_sum_memo(7, &vec![1,4,5], &mut memo).unwrap();
    println!("{:?}", result);
    assert_eq!(true, result.len() > 0);
    let mut memo = HashMap::new();
    let result = how_sum_memo(7, &vec![2,4,5], &mut memo).unwrap();
    println!("{:?}", result);
    assert_eq!(true, result.len() > 0);
    assert_eq!(true, how_sum_memo(7, &vec![2,4,5],&mut memo).unwrap().len() > 0);
    let mut memo = HashMap::new();
    assert_eq!(true , how_sum_memo(7, &vec![4,5],&mut memo).is_none());

    let mut memo = HashMap::new();
    assert_eq!(true , how_sum_memo(300, &vec![7,14],&mut memo).is_none());
}



/// How sum will get the slice of how to sum to targets

fn best_sum_memo(target: isize, numbers: &Vec<isize>, memo: &mut HashMap<isize, Option<Vec<isize>>>) -> Option<Vec<isize>>{
    match target{
        x if x < 0 => None ,
        x if x == 0 => Some(vec![]),
        _ => {
            match memo.get(&(target)){
                None => {}
                Some(v) => {
                    return v.clone();
                }
            }
            let mut shortest: Option<Vec<isize>> = None;
            for number in numbers{
                let key = target - number;
                if key < 0{
                    continue
                }
                let sum_result = best_sum_memo(key, numbers, memo);
                match sum_result{
                    Some(mut v) => {
                        if v.len() >= 0{
                            let mut new_vec:Vec<isize> = [*number].to_vec();
                            v.push(*number);

                            match shortest{
                                Some(ref sv) => {
                                    if sv.len() > v.len(){
                                        shortest = Some(v);
                                    }
                                },
                                _ => shortest = Some(v),
                            }
                        }
                    },
                    _ => {
                        // 只缓存处理后不存在序列的情况
                        memo.insert(key, None);
                    },
                };
            }
            return shortest;
        }
    }
}



#[test]
fn test_best_sum_memo(){
    let mut memo = HashMap::new();
    let result = best_sum_memo(7, &vec![1,4,5], &mut memo).unwrap();
    println!("{:?}", result);
    assert_eq!(true, result.len() > 0);
    let mut memo = HashMap::new();
    let result = best_sum_memo(7, &vec![2,4,5], &mut memo).unwrap();
    println!("{:?}", result);
    assert_eq!(true, result.len() > 0);
    assert_eq!(true, best_sum_memo(7, &vec![2,4,5],&mut memo).unwrap().len() > 0);
    let mut memo = HashMap::new();
    assert_eq!(true , best_sum_memo(7, &vec![4,5],&mut memo).is_none());

    let mut memo = HashMap::new();
    assert_eq!(true , best_sum_memo(300, &vec![7,14],&mut memo).is_none());
}

