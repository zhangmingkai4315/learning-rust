use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        } else {
            // left holds a right, and right holds a left, swap them.
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        }
    }

    // re-align left to account for the pivot at 0
    let left = left + 1;

    // place the pivot at its final location
    slice.swap(0, left - 1);

    // split_at_mut(mid: usize) -> (&mut [..mid), &mut [mid..])
    let (left, right) = slice.split_at_mut(left - 1);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        quicksort(slice)
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}



fn quick_sort_v2<T: Ord>(slice: &mut[T]){
    let len = slice.len();
    _quick_sort(slice, 0, len as isize-1);
}

fn _quick_sort<T: Ord>(slice: &mut[T], low: isize, high: isize){
    if low < high{
        let p = partition(slice, low, high);
        _quick_sort(slice, low, p - 1);
        _quick_sort(slice, p + 1, high);
    }
}

fn partition<T: Ord>(slice: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while slice[store_index as usize] < slice[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && slice[last_index as usize] > slice[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            slice.swap(store_index as usize, last_index as usize);
        }
    }
    slice.swap(store_index as usize, pivot as usize);
    store_index
}

#[test]
fn quick_sort_v2_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    quick_sort_v2(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}