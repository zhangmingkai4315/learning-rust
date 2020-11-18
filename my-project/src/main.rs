use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

fn main(){

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
    }

    // let r = 0x12345usize as *const i32;
    // unsafe {
    //     println!("{}", *r) // 不保证有效
    // }

    unsafe fn not_safe(){};

    unsafe {
        not_safe();
    }


    let mut v = vec![1,2,3,4,5];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    println!("{:?} / {:?}", a, b);
    // [1, 2, 3] / [4, 5]

    fn split_at_mut(slice :&mut [i32], mid: usize) -> (&mut[i32], &mut[i32]){
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(len >= mid);

        unsafe {
            (slice_from_raw_parts_mut(ptr, mid).as_mut().unwrap(),
             slice_from_raw_parts_mut(ptr.offset(mid as isize), len-mid)
                 .as_mut().unwrap())
        }
    }

    let (a, b) = split_at_mut(r, 3);
    println!("{:?} / {:?}", a, b);
    // [1, 2, 3] / [4, 5]

    static mut COUNTER : u32 = 0;

    unsafe {
        COUNTER = COUNTER + 1;
        println!("{}", COUNTER);
    }

    static COUNTER_2 : u32 = 1;
    println!("{}", COUNTER_2);

    struct Example{};
    unsafe {
        unsafe impl Send for Example{
        // 类型可以被安全的转移到多个线程
        }
        unsafe impl Sync for Example{
        // 类型可以被安全的多线程共享访问
        }
    }

}
