trait Sorter{
    fn sort<T>(&self, slice: &mut[T]) where T: Ord;
}


mod bubble_sort;
mod insert_sort;
mod select_sort;
mod quick_sort;

#[cfg(test)]
mod test{
    use super::*;
    use crate::bubble_sort::BubbleSort;
    use crate::insert_sort::InsertSort;
    use crate::select_sort::SelectSort;
    use crate::quick_sort::QuickSort;


    #[test]
    fn bubble_sort_work(){
        let mut slice = vec![5,4,3,2,1];
        BubbleSort.sort(&mut slice);
        assert_eq!(slice, [1,2,3,4,5]);
    }

    #[test]
    fn insert_sort_work(){
        let mut slice = vec![5,4,3,2,1];
        let insert_sort = InsertSort::new();
        insert_sort.sort(&mut slice);
        assert_eq!(slice, [1,2,3,4,5]);

        let mut slice = vec![5,4,3,2,1];
        let binary_insert_sort = InsertSort::new_binary();
        binary_insert_sort.sort(&mut slice);
        assert_eq!(slice, [1,2,3,4,5]);
    }

    #[test]
    fn select_sort_work(){
        let mut slice = vec![5,4,3,2,1];
        SelectSort.sort(&mut slice);
        assert_eq!(slice, [1,2,3,4,5]);
    }

    #[test]
    fn quick_sort_work(){
        let mut slice = vec![5,4,3,2,1];
        QuickSort.sort(&mut slice);
        assert_eq!(slice, [1,2,3,4,5]);
    }
}