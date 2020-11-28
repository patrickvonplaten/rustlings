// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let mut a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    let nice_slice_1 = &a[1..3];

    println!("Print out {:?} {:?}", nice_slice, nice_slice_1);
    assert_eq!([2, 3, 4], nice_slice);
}
