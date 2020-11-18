use crate::Sorter;

pub struct InsertSort{
    binary: bool
}
impl InsertSort{
    pub fn new()->InsertSort{
        InsertSort{
            binary: false
        }
    }
    pub fn new_binary()->InsertSort{
        InsertSort{
            binary: true
        }
    }
}

impl Sorter for InsertSort{
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        // 想象如何排序一副乱的扑克牌
        let size = slice.len();
        if self.binary{
            InsertSort::binary_sort(slice)
        }else{
            for unsorted in 1..size{
                // slice[unsorted..] not sorted
                // take slice[unsorted] and place in sorted location in slice[..=unsorted]
                let mut i = unsorted;
                while i>0 && slice[i-1] > slice[i] {
                    slice.swap(i-1, i);
                    i = i - 1;
                }

            }
        }

    }
}

impl InsertSort{
    fn binary_sort<T>( slice: &mut [T]) where T: Ord {
        let size = slice.len();
        for unsorted in 1..size{
            let i = slice[..unsorted].binary_search(&slice[unsorted]).unwrap_or_else(|i| i);
            slice[i..=unsorted].rotate_right(1);
        }
    }
}
