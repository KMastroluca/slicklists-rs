fn opaque_read(val: &i32) {
    println!("{}", val);
}

use std::cell::UnsafeCell;

fn main() {
    /*
    unsafe {

        let mut data = [0; 10];

        let slice1_all = &mut data[..]; // Slice for the entire array
        let ptr2_all = slice1_all.as_mut_ptr(); // Pointer For The Entire Array

        let ptr3_at_0 = ptr2_all; // Reference to 0th element in the array
        let ptr4_at_1 = ptr2_all.add(1); // Reference to 1th element
        let ref5_at_0 = &mut *ptr3_at_0; // Ptr to 1st element
        let ref6_at_1 = &mut *ptr4_at_1; // Ptr To 0th Element


        // A Jumbled Hash Of Pointer Usages
        *ref6_at_1 += 6;
        *ref5_at_0 += 5;
        *ptr4_at_1 += 4;
        *ptr3_at_0 += 3;


        // Modify All The Pointers In A Loop
        for idx in 0..10 {
            *ptr2_all.add(idx) += idx;
        }

        // Safe version of the above code
        for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
            *elem_ref += idx;
        }


        // Should Print [3, 3, 0, .....]
        println!("{:#?}", &data[..]);

    }
    */

    unsafe {
        let mut data = Box::new(10);
        let ptr1 = (&mut *data) as *mut i32;

        *ptr1 += 1;
        *data += 10;

        println!("{}", data);
    }
}
