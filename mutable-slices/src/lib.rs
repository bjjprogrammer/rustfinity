pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
    for num in slice.iter_mut() {
        if *num % 2 == 0 {
            *num *= 2;
        } else {
            *num -= 1;
        }
    }
}
