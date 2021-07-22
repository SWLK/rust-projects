// String slices are specific to strings,
// but there's a more general slice type
fn main() {
    let a = [1, 2, 3, 4, 5];

    // this slice has the type &[i32]
    // it works the same way as string slices do,
    // by storing a reference to the first element and a length
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
