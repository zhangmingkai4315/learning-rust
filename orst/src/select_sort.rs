use super::Sorter;

pub struct SelectSort;

impl Sorter for SelectSort{
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {

        let size = slice.len();
        for i in 0..size{

            let smallest_in_rest = slice[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)|v)
                .map(|(pos,_)| pos + i)
                .expect("is non-empty");
            
            // let smallest_in_rest = i + smallest_in_rest;

            // or

            // let mut smallest_in_rest = i;
            // for j in (i+1)..size{
            //     if slice[j] < slice[smallest_in_rest]{
            //         smallest_in_rest = j
            //     }
            // }
            slice.swap(smallest_in_rest, i);
        }
    }
}

