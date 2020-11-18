use super::Sorter;

pub struct QuickSort;

fn quick_sort<T:Ord>(slice: &mut[T]){
    let size = slice.len();
    if size == 1 || size == 0{
        return;
    }
    if size == 2{
        if slice[0] > slice[1]{
            slice.swap(0, 1);
        }
        return;
    }
    let (pivot, rest) = slice.split_first_mut().expect("slice is not- empty");
    let mut left = 0;
    let mut right = rest.len();

    while left != right {
        if &rest[left] <= pivot{
            left += 1;
        }else if &rest[right] > pivot{
            right -= 1;
        }else{
            println!("while swap {} {}", left, right);
            rest.swap(left, right);
            right -= 1;
            left += 1;
        }
        println!("while {} {}", left, right);
    }
    println!("{} {}", left, right);
    slice.swap(0, left);

    let (left, right) = slice.split_at_mut(left);

    quick_sort(left);
    quick_sort(&mut right[1..]);
    //[5,4,3,2,1]
}

impl Sorter for QuickSort{
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        quick_sort(slice)
    }
}