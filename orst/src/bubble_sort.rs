use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort{
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        let mut swapped = true;
        let size = slice.len();
        while swapped {
            swapped = false;
            for i in 0..size-1{
                if slice[i] > slice[i+1]{
                    slice.swap(i, i+1);
                    swapped = true;
                }
            }
        }
    }
}