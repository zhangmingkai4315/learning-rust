use std::fmt::Debug;

/// quick_select find the kth smallest T in slice
pub fn quick_select<T: PartialOrd + Debug + Clone>(
    dataset: &mut [T],
    start: usize,
    end: usize,
    k: usize,
) -> T {
    if k > 0 && k <= end - start + 1 {
        let j = partition(dataset, start, end);
        if j - start == k - 1 {
            return dataset[j].clone();
        } else if j - start > k - 1 {
            quick_select(dataset, start, j - 1, k - start)
        } else {
            quick_select(dataset, j + 1, end, k + start - j - 1)
        }
    } else {
        panic!("k must be smaller than size of dataset")
    }
}

pub fn partition<T: PartialOrd + Debug + Clone>(
    dataset: &mut [T],
    start: usize,
    end: usize,
) -> usize {
    if start == end {
        return start;
    }
    let partition_val = dataset[end].clone();
    let mut i = start;

    for j in start..=end - 1 {
        if dataset[j] <= partition_val {
            dataset.swap(i, j);
            i += 1;
        }
    }
    dataset.swap(i, end);
    return i;
}

#[cfg(test)]
mod test {
    use super::quick_select;
    use crate::quick_select::partition;

    #[test]
    fn partition_test() {
        let mut dataset = [4, 2, 1, 5, 3];
        let a = partition(&mut dataset, 0, 4);
        assert_eq!(dataset, [2, 1, 3, 5, 4]);
        assert_eq!(a, 2);

        let mut dataset = [4, 2, 1, 5, 6];
        let a = partition(&mut dataset, 0, 4);
        assert_eq!(dataset, [4, 2, 1, 5, 6]);
        assert_eq!(a, 4);

        let mut dataset = [5, 4, 3, 2, 1];
        let a = partition(&mut dataset, 0, 4);
        assert_eq!(dataset, [1, 4, 3, 2, 5]);
        assert_eq!(a, 0);
    }

    #[test]
    fn quick_select_test() {
        let mut dataset = [4, 2, 1, 5, 3];
        let a = quick_select(&mut dataset, 0, 4, 1);
        assert_eq!(a, 1);

        let mut dataset = [4, 2, 1, 5, 3];
        let a = quick_select(&mut dataset, 0, 4, 2);
        assert_eq!(a, 2);
        let mut dataset = [4, 2, 1, 5, 3];
        let a = quick_select(&mut dataset, 0, 4, 3);
        assert_eq!(a, 3);
        let mut dataset = [4, 2, 1, 5, 3];
        let a = quick_select(&mut dataset, 0, 4, 4);
        println!("{:?}", dataset);
        assert_eq!(a, 4);

        let mut dataset = [4, 2, 1, 5, 3];
        let a = quick_select(&mut dataset, 0, 4, 5);
        assert_eq!(a, 5);

        //
        // let a = quick_select(&mut dataset, 0, 4,4);
        // assert_eq!(a, 4);
    }
}
