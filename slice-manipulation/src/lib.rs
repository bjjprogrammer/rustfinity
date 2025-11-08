pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    for index in indices {
        if let Some(num) = slice.get_mut(*index) {
            *num = value;
        }
    }
}
